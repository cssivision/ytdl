use std::error::Error;
use std::collections::HashMap;
use url::{Url};
use chrono::{DateTime, Utc, Duration};
use format::Format;

struct VideoInfo<'a> {
    id: &'a str,
    title: &'a str,
    describe: &'a str,
    date_published: DateTime<Utc>,
    formats: Vec<Format<'a>>,
    keywords: Vec<&'a str>,
    author: &'a str,
    duration: Duration,
    html_player_file: &'a str,
}

const YOUTUBE_BASE_URL: &str = "https://www.youtube.com/watch";

pub fn get_video_info<'a>(value: &str) -> Result<VideoInfo<'a>, Box<Error>> {
    let parse_url = match Url::parse(value) {
        Ok(u) => u,
        Err(_) => {
            return get_info_from_id(value);
        },
    };

    let query: HashMap<String, String> = parse_url.query_pairs().into_owned().collect();
    Ok(10)
}

fn get_info_from_url() {

}

fn get_info_from_id<'a>(id: &str) -> Result<VideoInfo<'a>, Box<Error>> {
    let mut parse_url = Url::parse(YOUTUBE_BASE_URL)?;
    parse_url.set_query(Some(format!("v={}", id).as_str()));
    println!("{}", parse_url.as_str());
    Ok(10)
}