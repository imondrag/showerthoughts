use app_dirs::{app_root, AppDataType, AppInfo};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
type BoxResult<T> = Result<T, Box<dyn Error>>;

pub const APP_INFO: AppInfo = AppInfo {
    name: env!("CARGO_PKG_NAME"),
    author: env!("CARGO_PKG_AUTHORS"),
};

const REDDIT_URL: &'static str =
    "https://www.reddit.com/r/showerthoughts/top.json?sort=top&t=week&limit=100";

const REDDIT_API_PARAMS: &[(&'static str, &'static str)] =
    &[("sort", "top"), ("t", "week"), ("limit", "100")];

// set to 12 hours
const CACHE_INVALIDATION_TIMEOUT: Duration = Duration::from_secs(60 * 60 * 12);

lazy_static! {
    static ref CACHE_PATH: PathBuf = {
        let mut cache = app_root(AppDataType::UserCache, &APP_INFO)
            .expect("ERROR: Could not create cache");
        cache.push("list.bin");
        cache
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditApiResponse {
    pub data: RedditData,
    pub expires_at: Option<SystemTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditData {
    pub children: Vec<RedditSingletonResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditSingletonResponse {
    pub data: RedditSingletonData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditSingletonData {
    pub author: String,
    pub title: String,
}

pub fn update_titles() -> BoxResult<RedditApiResponse> {
    let client = reqwest::Client::new();
    let mut res = client.post(REDDIT_URL).json(REDDIT_API_PARAMS).send()?;

    let mut parsed: RedditApiResponse = res.json()?;
    parsed.expires_at = Some(SystemTime::now() + CACHE_INVALIDATION_TIMEOUT);

    write_cache_to_file(&parsed)?;
    Ok(parsed)
}

pub fn read_cache_from_file() -> BoxResult<(RedditApiResponse, bool)> {
    // Open the file in read-only mode.
    let fin = File::open(CACHE_PATH.as_path())?;

    // Buffer while reading file to reduce syscalls
    let fin = BufReader::new(fin);

    let cache: RedditApiResponse = bincode::deserialize_from(fin)?;
    let is_expired = cache.expires_at.map_or(true, |t| t > SystemTime::now());

    Ok((cache, is_expired))
}

pub fn write_cache_to_file(
    cache: &RedditApiResponse,
) -> Result<(), impl Error> {
    // Create/truncate the file
    let fout = File::create(CACHE_PATH.as_path())?;

    // Buffer while writing file to reduce syscalls
    let fout = BufWriter::new(fout);

    bincode::serialize_into(fout, cache)
}
