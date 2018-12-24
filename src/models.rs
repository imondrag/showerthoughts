use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedPosts {
    pub posts: Vec<RedditSingletonResponse>,
    pub expires_at: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditApiResponse {
    pub data: RedditData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditData {
    pub children: Vec<RedditSingletonResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditSingletonResponse {
    pub data: RedditPost,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditPost {
    pub author: String,
    pub title: String,
}
