use rand::seq::SliceRandom;
use showerthoughts::{
    read_cache_from_file, update_titles, RedditApiResponse,
    RedditSingletonResponse, APP_INFO,
};

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

    let api_res: RedditApiResponse = {
        if let Ok((cache, is_expired)) = read_cache_from_file() {
            if !is_expired {
                cache
            } else {
                update_titles().unwrap_or(cache)
            }
        } else {
            update_titles().expect("Error fetching response")
        }
    };

    let mut rng = rand::thread_rng();
    let post: &RedditSingletonResponse = api_res
        .data
        .children
        .choose(&mut rng)
        .expect("Error choosing post");
    println!("\n\"{}\"\n\t-{}", post.data.title, post.data.author);
}
