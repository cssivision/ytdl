# ytdl
[![Build Status](https://img.shields.io/travis/cssivision/ytdl.svg?style=flat-square)](https://travis-ci.org/cssivision/ytdl)
[![crate](https://img.shields.io/crates/v/ytdl.svg)](https://crates.io/crates/ytdl)
[![License](http://img.shields.io/badge/license-mit-blue.svg)](https://github.com/cssivision/ytdl/blob/master/LICENSE)

a simple cli for downloading youtube video.

# Installation
use cargo.
```sh
cargo install ytdl
```

Fetch [lastest releases](https://github.com/cssivision/ytdl/releases).
## macos
```sh
wget https://github.com/cssivision/ytdl/releases/download/v0.1.5/ytdl
chmod +x ytdl
```
## linux
```sh
wget https://github.com/cssivision/ytdl/releases/download/v0.1.5/ytdl.linux
chmod +x ytdl.linux
```
# Usage
## example
```sh
ytdl https://www.youtube.com/watch?v=GbWECt0M3CI
```
```
USAGE:
    ytdl [FLAGS] [OPTIONS] <url>

FLAGS:
    -a, --append          append to output file instead of overwriting
    -d, --debug           output debug log
    -u, --download-url    prints download url to stdout
    -h, --help            Prints help information
    -i, --info            only output info
    -j, --json            print info json to stdout
        --no-progress     write output to a file
    -s, --silent          only output error, also diables progressbar
    -V, --version         Prints version information

OPTIONS:
    -f, --filter <FILTER>...            filter available formats, syntax: val1 val2 val3
    -o, --output <FILE>                 write output to a file
    -r, --range <RANGE>                 download a specific range of bytes of the video, [start]-[end]
        --start-offset <STARTOFFSET>    offset the start of the video

ARGS:
    <url>    youtube video url, short url or video id
```

# Licenses

All source code is licensed under the [MIT License](https://github.com/cssivision/ytdl/blob/master/LICENSE).

# Todo 
- concurrent download.

