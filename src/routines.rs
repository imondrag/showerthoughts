pub use crate::models::*;
use app_dirs::{app_root, AppDataType, AppInfo};
use lazy_static::lazy_static;
#[allow(unused)]
use log::debug;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub const APP_INFO: AppInfo = AppInfo {
    name: env!("CARGO_PKG_NAME"),
    author: env!("CARGO_PKG_AUTHORS"),
};

const REDDIT_SOURCES: &[&'static str] = &["showerthoughts", "todayilearned"];

const REDDIT_API_QUERY: &[(&'static str, &'static str)] =
    &[("sort", "top"), ("t", "week"), ("limit", "100")];

// set to 12 hours
const CACHE_INVALIDATION_TIMEOUT: Duration = Duration::from_secs(60 * 60 * 12);

lazy_static! {
    static ref CACHE_PATH: PathBuf = {
        let mut cache =
            app_root(AppDataType::UserCache, &APP_INFO).expect("ERROR: could not create cache");
        cache.push("list.bin");
        cache
    };
}

/*
lazy_static! {
    static ref USER_CONFIG: UserConfig = {
        let mut config = app_root(AppDataType::UserConfig, &APP_INFO)
            .expect("ERROR: could not create config file");
        config.push("config.toml");
        config
    };
}
*/

pub fn update_titles() -> Result<CachedPosts, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://www.reddit.com/r/{}/top.json",
        REDDIT_SOURCES.join("+")
    );

    let mut res = client.post(&url).json(REDDIT_API_QUERY).send()?;

    let parsed: RedditApiResponse = res.json()?;
    let expires_at = SystemTime::now() + CACHE_INVALIDATION_TIMEOUT;

    let cache = CachedPosts {
        posts: parsed.data.children,
        expires_at,
    };

    write_cache_to_file(&cache)?;
    Ok(cache)
}

pub fn read_cache_from_file() -> Result<(CachedPosts, bool), Box<dyn Error>> {
    // Open the file in read-only mode.
    let fin = File::open(CACHE_PATH.as_path())?;

    // Buffer while reading file to reduce syscalls
    let fin = BufReader::new(fin);

    let cache: CachedPosts = bincode::deserialize_from(fin)?;
    let is_expired = cache.expires_at > SystemTime::now();

    Ok((cache, is_expired))
}

pub fn write_cache_to_file(cache: &CachedPosts) -> Result<(), impl Error> {
    // Create/truncate the file
    let fout = File::create(CACHE_PATH.as_path())?;

    // Buffer while writing file to reduce syscalls
    let fout = BufWriter::new(fout);

    bincode::serialize_into(fout, cache)
}
