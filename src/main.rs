extern crate clap;
#[macro_use]
extern crate log;

use clap::{App, AppSettings, Arg};

mod format;

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
    start_offset: String,
}

fn main() {
    let flags = vec![
        Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("write output to a file")
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
        filter = matches.values_of("filter").unwrap().map(|s| String::from(s)).collect();
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
        start_offset: matches.value_of("start-offset").unwrap_or_default().to_string(),
    };

    let identifier = matches.value_of("url").unwrap_or_default().to_string();
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

fn handler(identifier: String, options: &Options) {
    if options.output_file == "" {
    }
}