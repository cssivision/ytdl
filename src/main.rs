extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate env_logger;
extern crate url;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate pbr;

use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

use clap::{App, AppSettings, Arg};
use pbr::{ProgressBar, Units};
use reqwest::header::ContentLength;

mod format;
mod video_info;

use video_info::YTDL_PROXY_URL;

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
    env_logger::init().expect("env logger init fail");

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
            .help("download a specific range of bytes of the video, [start]-[end]")
            .takes_value(true),
        Arg::with_name("url")
            .help("youtube url")
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
            .help("only output info")];
        
    let matches = App::new("ytdl")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.0.1")
        .about("download youtube videos")
        .args(&flags)
        .get_matches();

    
    let mut filter = vec![];
    if matches.is_present("filter") {
        filter = matches.values_of("filter").unwrap().map(|x| x.to_string()).collect();
    }

    let mut options = Options {
        no_progress: matches.is_present("no-progress"),
        info_only: matches.is_present("info"),
        silent: matches.is_present("silent"),
        debug: matches.is_present("debug"),
        append: matches.is_present("append"),
        json: matches.is_present("json"),
        download_url: matches.is_present("download-url"),
        filter: filter,
        output_file: matches.value_of("output").unwrap_or_default().to_string(),
        byte_range: matches.value_of("range").unwrap_or_default().to_string(),
        start_offset: matches.value_of("start-offset").unwrap_or("0").parse::<i32>().unwrap(),
        proxy_url: matches.value_of("proxy-url").unwrap_or_default().to_string(),
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
        return
    } else if options.json {
        println!("{}", serde_json::to_string(&info).unwrap_or_default());
        return 
    }

    let formats = &info.formats;
    for x in &options.filter {

    }

    if formats.len() == 0 {
        println!("no formats available that match criteria");
        return 
    }

    let mut download_url = match video_info::get_download_url(&formats[0]) {
        Ok(u) => u,
        Err(e) => {
            println!("unable to get download url: {}", e.to_string());
            return
        }
    };

    if options.start_offset != 0 {
        download_url.query_pairs_mut().append_pair("begin", &format!("{}", &options.start_offset * 1000));
    }

    if options.download_url {
        println!("{}", download_url.as_str());
    }

    let filename = video_info::get_filename(&info, &formats[0]);
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

    let client = video_info::get_client().expect("get request client fail");
    let mut resp = client
        .get(download_url.as_str())
        .expect("download fail")
        .send()
        .expect("download fail");
    
    let file_size = resp.headers().get::<ContentLength>()
                .map(|l| **l)
                .unwrap_or(0);

    let mut pb = ProgressBar::new(file_size);
    pb.format("╢▌▌░╟");
    pb.set_units(Units::Bytes);
    pb.show_percent = true;
    pb.show_speed = true;
    pb.show_time_left = true;
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
}

fn parse_filter() {
    
}