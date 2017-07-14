use std::error::Error;
use std::collections::HashMap;
use std::io::Read;

use url::{Url, form_urlencoded};
use chrono::{DateTime, Utc, Duration};
use format::Format;
use reqwest::{self as request, StatusCode};

#[derive(Default)]
pub struct VideoInfo {
    id: String,
    title: String,
    describe: String,
    date_published: Option<DateTime<Utc>>,
    formats: Vec<Format>,
    keywords: Vec<String>,
    author: Option<String>,
    duration: Option<Duration>,
    html_player_file: String,
}

const YOUTUBE_BASE_URL: &str = "https://www.youtube.com/watch";
const YOUTUBE_VIDEO_INFO_URL: &str = "https://www.youtube.com/get_video_info";

pub fn get_video_info(value: &str) -> Result<VideoInfo, Box<Error>> {
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
}

fn get_info_from_url(u: &Url) -> Result<VideoInfo, Box<Error>> {
    Err(From::from("vdfvf"))
}

fn get_info_from_id(id: &str) -> Result<VideoInfo, Box<Error>> {
    let mut parse_url = Url::parse(YOUTUBE_BASE_URL)?;
    parse_url.set_query(Some(format!("v={}", id).as_str()));
    // let mut resp = reqwest::get(parse_url.as_str())?;
    let client = request::Client::builder()?
        .proxy(request::Proxy::all("http://127.0.0.1:1087")?)
        .build()?;
    let mut resp = client.get(parse_url.as_str())?.send()?;

    if resp.status() != StatusCode::Ok {
        return Err(From::from("Invalid status code"))
    }
    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    get_video_info_from_html(id, &body)
}

fn get_video_info_from_short_url(u: &Url) -> Result<VideoInfo, Box<Error>> {
    Err(From::from("vdfvf"))
}

fn get_video_info_from_url(u: &Url) -> Result<VideoInfo, Box<Error>> {
    Err(From::from("vdfvf"))
}

fn get_video_info_from_html(id: &str, body: &str) -> Result<VideoInfo, Box<Error>> {
    let info_url = format!("{}?video_id={}", YOUTUBE_VIDEO_INFO_URL, id);
    println!("{}", info_url);
    let client = request::Client::builder()?
        .proxy(request::Proxy::all("http://127.0.0.1:1087")?)
        .build()?;
    let mut resp = client.get(info_url.as_str())?.send()?;
    if resp.status() != StatusCode::Ok {
        return Err(From::from("video info response invalid status code"));
    }

    let mut info = String::new();
    resp.read_to_string(&mut info)?;
    let info = parse_query(info);
    let mut video_info = VideoInfo{..Default::default()};
    match info.get("status") {
        Some(s) => {
            if s == "fail" {
                return Err(
                    From::from(format!(
                        "Error {}:{}", 
                        info.get("errorcode").unwrap_or(&"".to_string()), 
                        info.get("reason").unwrap_or(&"".to_string()))
                    )
                );
            } 
        },
        None => {
            return Err(From::from("get video info, status not found"));
        }
    };

    if let Some(author) = info.get("author") {
        video_info.author = Some(author.to_string());
    }
    debug!("unable to extract author");

    if let Some(duration) = info.get("length_seconds") {
        // video_info.duration = 
    }
    debug!("unable to parse duration string");
    Ok(video_info)
}

fn download() {
    
}

fn parse_query(query_str: String) -> HashMap<String, String> {
    let parse_query = form_urlencoded::parse(query_str.as_bytes());
    return parse_query.into_owned().collect::<HashMap<String, String>>();
}