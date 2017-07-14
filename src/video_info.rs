use std::error::Error;
use std::collections::HashMap;
use std::io::Read;

use url::{Url};
use chrono::{DateTime, Utc, Duration};
use format::Format;
use reqwest::{self, StatusCode};

pub struct VideoInfo<'a> {
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

    if parse_url.host_str() == Some("youtu.be") {
        return get_video_info_from_short_url(&parse_url);
    }

    return get_info_from_url(&parse_url);

    let query: HashMap<String, String> = parse_url.query_pairs().into_owned().collect();
    Err(From::from("Identifier type must be a string"))
}

fn get_info_from_url<'a>(u: &Url) -> Result<VideoInfo<'a>, Box<Error>> {
    Err(From::from("vdfvf"))
}

fn get_info_from_id<'a>(id: &str) -> Result<VideoInfo<'a>, Box<Error>> {
    let mut parse_url = Url::parse(YOUTUBE_BASE_URL)?;
    parse_url.set_query(Some(format!("v={}", id).as_str()));
    // let mut resp = reqwest::get(parse_url.as_str())?;
    let client = reqwest::Client::builder()?
        .proxy(reqwest::Proxy::all("https://127.0.0.1:1087")?)
        .build()?;

    let mut resp = client.get(parse_url.as_str())?.send()?;
    if resp.status() != StatusCode::Ok {
        return Err(From::from("Invalid status code"))
    }
    let mut body = String::new();
    resp.read_to_string(&mut body);
    get_video_info_from_html(id, &body)
}

fn get_video_info_from_short_url<'a>(u: &Url) -> Result<VideoInfo<'a>, Box<Error>> {
    Err(From::from("vdfvf"))
}

fn get_video_info_from_url<'a>(u: &Url) -> Result<VideoInfo<'a>, Box<Error>> {
    Err(From::from("vdfvf"))
}

fn get_video_info_from_html<'a>(id: &str, body: &str) -> Result<VideoInfo<'a>, Box<Error>> {
    Err(From::from("no error"))
}