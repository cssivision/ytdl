use std::error::Error;
use std::collections::HashMap;
use std::io::Read;
use std::time::{Duration as StdDuration};

use url::{Url, form_urlencoded};
use chrono::{DateTime, Utc, Duration};
use format::Format;
use reqwest::{self as request, StatusCode};

#[derive(Default)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub describe: String,
    pub date_published: Option<DateTime<Utc>>,
    pub formats: Vec<Format>,
    pub keywords: Vec<String>,
    pub author: String,
    pub duration: Option<Duration>,
    pub html_player_file: String,
}

const YOUTUBE_BASE_URL: &str = "https://www.youtube.com/watch";
const YOUTUBE_VIDEO_INFO_URL: &str = "https://www.youtube.com/get_video_info";
const YOUTUBE_VIDEO_INFO_EURL: &str = "https://youtube.googleapis.com/v/";

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
    let mut video_info: VideoInfo = Default::default();
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
        video_info.author = author.to_string();
    } else {
        debug!("unable to extract author");
    }

    if let Some(length) = info.get("length_seconds") {
        let duration = length.parse::<u64>()?;
        video_info.duration = Some(Duration::from_std(StdDuration::new(duration, 0))?);
    } else {
        debug!("unable to parse duration string");
    }

    if let Some(keywords) = info.get("keywords") {
        video_info.keywords = keywords.split(",").map(|s| s.to_string()).collect();
    } else {
        debug!("unable to extract keywords")
    }

    let mut format_strings = vec![];
    if let Some(fmt_stream) = info.get("url_encoded_fmt_stream_map") {
        format_strings.append(&mut fmt_stream.split(",").collect())
    }

    if let Some(adaptive_fmts) = info.get("adaptive_fmts") {
        format_strings.append(&mut adaptive_fmts.split(",").collect());
    }

    for v in &format_strings {
        let query = parse_query(v.to_string());
        let itag = match query.get("itag") {
            Some(i) => {

            },
            None => {
                continue;
            }
        };
    }

    Ok(video_info)
}

fn download() {
    
}

fn parse_query(query_str: String) -> HashMap<String, String> {
    let parse_query = form_urlencoded::parse(query_str.as_bytes());
    return parse_query.into_owned().collect::<HashMap<String, String>>();
}