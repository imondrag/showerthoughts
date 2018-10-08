extern crate bincode;
extern crate dirs;
extern crate futures;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use dirs::cache_dir;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

type BoxResult<T> = Result<T, Box<dyn Error>>;

lazy_static! {
    static ref CACHE_PATH: PathBuf = {
        let mut cache = cache_dir().unwrap();
        cache.push("showerthoughts");
        cache.set_file_name("list.bin");
        cache
    };
}

const REDDIT_URL: &'static str =
    "https://www.reddit.com/r/showerthoughts/top.json?sort=top&t=week&limit=100";

const CACHE_INVALIDATION_TIMEOUT: Duration = Duration::from_secs(60 * 60 * 8);

fn main() {
    let api_res: RedditApiResponse = {
        if let Ok((cache, is_expired)) = read_cache_from_file() {
            if !is_expired {
                cache
            } else {
                update_titles().unwrap_or(cache)
            }
        } else {
            update_titles().expect("COULD NOT CONNECT TO REDDIT API")
        }
    };

    let post: &RedditSingletonResponse =
        thread_rng().choose(&api_res.data.children).unwrap();
    println!("\n\"{}\"\n\t-{}", post.data.title, post.data.author);
}

fn update_titles() -> BoxResult<RedditApiResponse> {
    let mut res = reqwest::get(REDDIT_URL)?;
    let mut parsed: RedditApiResponse = res.json()?;
    parsed.created = Some(SystemTime::now());

    write_cache_to_file(&parsed)?;
    Ok(parsed)
}

fn read_cache_from_file() -> BoxResult<(RedditApiResponse, bool)> {
    // Open the file in read-only mode.
    let fin = File::open(CACHE_PATH.as_path())?;

    // Buffer while reading file to reduce syscalls
    let fin = BufReader::new(fin);

    let cache: RedditApiResponse = bincode::deserialize_from(fin)?;

    let mut is_expired = true;
    if let Some(created) = cache.created {
        if created.elapsed()? > CACHE_INVALIDATION_TIMEOUT {
            is_expired = false;
        }
    }

    Ok((cache, is_expired))
}

fn write_cache_to_file(cache: &RedditApiResponse) -> Result<(), impl Error> {
    // create dirs if they don't exist
    std::fs::create_dir_all(CACHE_PATH.parent().unwrap())?;

    // Create/truncate the file
    let fout = File::create(CACHE_PATH.as_path())?;

    // Buffer while writing file to reduce syscalls
    let fout = BufWriter::new(fout);

    bincode::serialize_into(fout, cache)
}

#[derive(Debug, Serialize, Deserialize)]
struct RedditApiResponse {
    data: RedditData,
    created: Option<SystemTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RedditData {
    children: Vec<RedditSingletonResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RedditSingletonResponse {
    data: RedditSingletonData,
}

#[derive(Debug, Serialize, Deserialize)]
struct RedditSingletonData {
    author: String,
    title: String,
}
