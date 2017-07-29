#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate ytdl;
extern crate clap;
extern crate url;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;
extern crate pbr;
extern crate serde_json;
extern crate openssl_probe;

use clap::{App, AppSettings, Arg};
use pbr::{ProgressBar, Units};
use reqwest::header::{ContentLength, Headers, Range};
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::str::FromStr;
use ytdl::format;
use ytdl::format_list::{Filter, FormatList};
use ytdl::video_info::{self, YTDL_PROXY_URL};

#[derive(Debug)]
struct Options {
    no_progress: bool,
    info_only: bool,
    silent: bool,
    debug: bool,
    append: bool,
    json: bool,
    download_url: bool,
    filter: Vec<String>,
    byte_range: String,
    output_file: String,
    start_offset: i32,
    proxy_url: String,
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    env_logger::init().unwrap();

    let flags = vec![
        Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("write output to a file")
            .takes_value(true),
        Arg::with_name("proxy-url")
            .short("p")
            .long("proxy-url")
            .value_name("PROXY_URL")
            .help("use proxy for the request")
            .takes_value(true),
        Arg::with_name("no-progress")
            .long("no-progress")
            .help("write output to a file"),
        Arg::with_name("range")
            .short("r")
            .long("range")
            .value_name("RANGE")
            .help(
                "download a specific range of bytes of the video, [start]-[end]",
            )
            .takes_value(true),
        Arg::with_name("url")
            .help("youtube video url, short url or video id")
            .required(true)
            .index(1),
        Arg::with_name("download-url")
            .short("-u")
            .long("download-url")
            .help("prints download url to stdout"),
        Arg::with_name("json")
            .short("j")
            .long("json")
            .help("print info json to stdout"),
        Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("output debug log"),
        Arg::with_name("filter")
            .short("f")
            .long("filter")
            .value_name("FILTER")
            .multiple(true)
            .help("filter available formats, syntax: val1 val2 val3")
            .takes_value(true),
        Arg::with_name("append")
            .short("-a")
            .long("--append")
            .help("append to output file instead of overwriting"),
        Arg::with_name("start-offset")
            .long("start-offset")
            .value_name("STARTOFFSET")
            .help("offset the start of the video")
            .takes_value(true),
        Arg::with_name("silent")
            .short("s")
            .long("silent")
            .help("only output error, also diables progressbar"),
        Arg::with_name("info")
            .short("i")
            .long("info")
            .help("only output info"),
    ];

    let matches = App::new("ytdl")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.4")
        .about("download youtube videos")
        .args(&flags)
        .get_matches();

    let mut options = Options {
        no_progress: matches.is_present("no-progress"),
        info_only: matches.is_present("info"),
        silent: matches.is_present("silent"),
        debug: matches.is_present("debug"),
        append: matches.is_present("append"),
        json: matches.is_present("json"),
        download_url: matches.is_present("download-url"),
        filter: matches
            .values_of("filter")
            .unwrap_or_default()
            .map(|x| x.to_string())
            .collect(),
        output_file: matches.value_of("output").unwrap_or_default().to_string(),
        byte_range: matches.value_of("range").unwrap_or_default().to_string(),
        start_offset: matches
            .value_of("start-offset")
            .unwrap_or("0")
            .parse::<i32>()
            .unwrap(),
        proxy_url: matches
            .value_of("proxy-url")
            .unwrap_or_default()
            .to_string(),
    };

    if !options.proxy_url.is_empty() {
        env::set_var(YTDL_PROXY_URL, &options.proxy_url);
    }

    let identifier = matches.value_of("url").unwrap_or_default();
    if options.filter.is_empty() {
        options.filter = vec![
            format!("{}:mp4", format::FORMAT_EXTENSION_KEY),
            format!("!{}:", format::FORMAT_VIDEO_ENCODING_KEY),
            format!("!{}:", format::FORMAT_AUDIO_ENCODING_KEY),
            format!("best"),
        ];
    }

    handler(identifier, &options);
}

fn handler(identifier: &str, options: &Options) {
    info!("fetching video info...");
    let info = match video_info::get_video_info(identifier) {
        Ok(i) => i,
        Err(e) => {
            println!("unable to fetch video info: {}", e.to_string());
            return;
        }
    };

    if options.info_only {
        println!("Author: {}", info.author);
        println!("Duration: {}s", info.duration);
        return;
    } else if options.json {
        println!("{}", serde_json::to_string(&info).unwrap_or_default());
        return;
    }

    let formats = filter_formats(&options.filter, &info.formats);
    if formats.is_empty() {
        println!("no formats available that match criteria");
        return;
    }

    let mut download_url = match video_info::get_download_url(&formats[0]) {
        Ok(u) => u,
        Err(e) => {
            println!("unable to get download url: {}", e.to_string());
            return;
        }
    };

    if options.start_offset != 0 {
        download_url
            .query_pairs_mut()
            .append_pair("begin", &format!("{}", &options.start_offset * 1000,));
    }

    if options.download_url {
        println!("{}", download_url.as_str());
    }


    let mut filename = if !options.output_file.is_empty() {
        format!("{}.{}", options.output_file, &formats[0].extension)
    } else {
        video_info::get_filename(&info, &formats[0])
    };
    filename = filename.replace("/", "-");

    let mut file = if options.append {
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&filename)
            .expect("create output file fail")
    } else {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&filename)
            .expect("create output file fail")
    };

    info!("download to {}", filename);
    let mut headers = Headers::new();
    if !options.byte_range.is_empty() {
        let r =
            Range::from_str(&format!("bytes={}", &options.byte_range)).expect("invalid range str");
        headers.set(r);
    }

    let client = video_info::get_client().expect("get request client fail");
    let mut resp = client
        .get(download_url.as_str())
        .expect("download fail")
        .headers(headers)
        .send()
        .expect("download fail");

    let file_size = resp.headers()
        .get::<ContentLength>()
        .map(|l| **l)
        .unwrap_or(0);

    let mut pb = ProgressBar::new(file_size);
    pb.format("╢▌▌░╟");
    pb.set_units(Units::Bytes);
    pb.show_percent = true;
    pb.show_speed = true;
    pb.show_time_left = true;
    pb.set_width(Some(100));
    let mut buf = [0; 128 * 1024];

    loop {
        match resp.read(&mut buf) {
            Ok(len) => {
                file.write_all(&buf[..len]).expect("write to file fail");
                if !options.silent && !options.no_progress {
                    pb.add(len as u64);
                }
                if len == 0 {
                    break;
                }
            }
            Err(e) => panic!("{}", e.to_string()),
        };
    }

    println!("");
}

fn filter_formats(filters: &Vec<String>, formats: &FormatList) -> FormatList {
    let mut formats = formats.clone();
    for fi in filters {
        let filter_str = fi.as_str();
        formats = match filter_str {
            "best" | "worst" => {
                formats
                    .extremes(format::FORMAT_RESOLUTION_KEY, filter_str == "best")
                    .extremes(format::FORMAT_AUDIO_BITRATE_KEY, filter_str == "best")
            }
            "best-video" | "worst-video" => {
                formats.extremes(
                    format::FORMAT_RESOLUTION_KEY,
                    filter_str.starts_with("best"),
                )
            }
            "best-audio" | "worst-audio" => {
                formats.extremes(
                    format::FORMAT_AUDIO_BITRATE_KEY,
                    filter_str.starts_with("best"),
                )
            }
            _ => {
                let split = filter_str.splitn(2, ":").collect::<Vec<&str>>();
                if split.len() != 2 {
                    panic!("invalid filter key");
                }

                let mut key = split[0].to_string();
                let exclude = key.starts_with("!");
                let start = if exclude { 1 } else { 0 };
                let key: String = key.drain(start..).collect::<String>();
                let value = split[1].trim();
                if value == "best" || value == "worst" {
                    let f = formats.extremes(&key, value == "best");
                    formats = formats.subtract(&f);
                    return formats;
                }
                let mut vals = value.split(",").collect::<Vec<&str>>();
                for i in 0..vals.len() {
                    vals[i] = vals[i].trim();
                }
                let f = formats.filter(&key, &vals);
                if exclude {
                    formats.subtract(&f)
                } else {
                    f
                }
            }
        };
    }
    formats
}
