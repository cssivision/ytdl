use std::error::Error;
use std::collections::HashMap;
use std::io::Read;
use std::env;

use url::{Url, form_urlencoded};
use format::Format;
use reqwest::{self as request, StatusCode, Client};

#[derive(Default)]
pub struct VideoInfo {
    pub id: String,
    pub describe: String,
    pub formats: Vec<Format>,
    pub keywords: Vec<String>,
    pub author: String,
    pub duration: i32,
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

    return get_video_info_from_url(&parse_url);
}

fn get_video_info_from_url(u: &Url) -> Result<VideoInfo, Box<Error>> {
    if let Some(video_id) = u.query_pairs().into_owned().collect::<HashMap<String, String>>().get("v") {
        return get_info_from_id(video_id);
    }
    Err(From::from("invalid youtube url, no video id"))
}

fn get_info_from_id(id: &str) -> Result<VideoInfo, Box<Error>> {
    let mut parse_url = Url::parse(YOUTUBE_BASE_URL)?;
    parse_url.set_query(Some(format!("v={}", id).as_str()));
    let client = get_client()?;
    let mut resp = client.get(parse_url.as_str())?.send()?;

    if resp.status() != StatusCode::Ok {
        return Err(From::from("Invalid status code"))
    }
    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    get_video_info_from_html(id, &body)
}

fn get_video_info_from_short_url(u: &Url) -> Result<VideoInfo, Box<Error>> {
    let path = u.path().trim_left_matches("/");
    if path.len() > 0 {
        return get_info_from_id(path);
    }

    Err(From::from("could not parse short URL"))
}

fn get_video_info_from_html(id: &str, body: &str) -> Result<VideoInfo, Box<Error>> {
    let info_url = format!("{}?video_id={}", YOUTUBE_VIDEO_INFO_URL, id);
    debug!("{}", info_url);
    let client = get_client()?;
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
        video_info.duration = length.parse::<i32>().unwrap_or_default();
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

    let mut formats: Vec<Format> = vec![];
    for v in &format_strings {
        let query = parse_query(v.to_string());
        let itag = match query.get("itag") {
            Some(i) => i,
            None => {
                continue;
            }
        };

        if let Ok(i) = itag.parse::<i32>() {
            if let Some(mut f) = Format::new(i) {
                if query.get("conn").unwrap_or(&"".to_string()).starts_with("rtmp") {
                    f.meta.insert("rtmp".to_string(), "true".to_string());
                }

                for (k, v) in &query {
                    f.meta.insert(k.to_string(), v.to_string());
                }

                formats.push(f);
            } else {
                debug!("no metadata found for itag: {}, skipping...", itag)
            }
        }
    }

    video_info.formats = formats;
    Ok(video_info)
}

fn download() {
    
}

fn parse_query(query_str: String) -> HashMap<String, String> {
    let parse_query = form_urlencoded::parse(query_str.as_bytes());
    return parse_query.into_owned().collect::<HashMap<String, String>>();
}

fn get_client() -> Result<Client, Box<Error>> {
    let proxy_url = env::var(super::YTDL_PROXY_URL)?;
    let client: Client;
    if proxy_url.is_empty() {
        client = request::Client::new()?;
    } else {
        client = request::Client::builder()?
            .proxy(request::Proxy::all(proxy_url.as_str())?)
            .build()?;
    }
    
    Ok(client)
}