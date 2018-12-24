use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

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
