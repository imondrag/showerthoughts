mod models;
mod routines;

use crate::routines::*;
use rand::prelude::*;

fn load_posts() -> Vec<RedditSingletonResponse> {
    let cache = if let Ok((cache, is_expired)) = read_cache_from_file() {
        if !is_expired {
            cache
        } else {
            update_titles().unwrap_or(cache)
        }
    } else {
        update_titles().expect("Error fetching response")
    };

    cache.posts
}

fn main() {
    // On run, let's check if we've cached a response beforehand
    //  if we have a cached response, check to see if it is still recent enough
    //      if it's recent, use it,
    //      otherwise fetch another response and cache it
    //
    //      if the fetch fails,
    //      use the expired cached response anyway
    //
    //  if we don't have a cached response, fetch one and cache it
    //      if the fetch fails, panic!('Error fetching response')
    //
    //  print randomly chosen value from response

    pretty_env_logger::init();

    let res_vec = load_posts();
    let mut rng = rand::thread_rng();

    let post: &RedditPost = res_vec
        .choose(&mut rng)
        .map(|w_data| &w_data.data) // unwrap the data field as a single post
        .expect("Error choosing post");

    println!("\"{}\"\n\t-{}", post.title, post.author);
}
