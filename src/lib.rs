mod models;

use crate::models::*;
use app_dirs::{app_root, AppDataType, AppInfo};
use lazy_static::lazy_static;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub use crate::models::RedditSingletonResponse;

pub const APP_INFO: AppInfo = AppInfo {
    name: env!("CARGO_PKG_NAME"),
    author: env!("CARGO_PKG_AUTHORS"),
};

const REDDIT_URL: &'static str =
    "https://www.reddit.com/r/showerthoughts/top.json";

const REDDIT_API_QUERY: &[(&'static str, &'static str)] =
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

pub fn update_titles() -> Result<RedditApiResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut res = client.post(REDDIT_URL).json(REDDIT_API_QUERY).send()?;

    let mut parsed: RedditApiResponse = res.json()?;
    parsed.expires_at = Some(SystemTime::now() + CACHE_INVALIDATION_TIMEOUT);

    write_cache_to_file(&parsed)?;
    Ok(parsed)
}

pub fn read_cache_from_file(
) -> Result<(RedditApiResponse, bool), Box<dyn Error>> {
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
