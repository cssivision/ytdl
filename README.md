# ytdl
[![crate](https://img.shields.io/crates/v/ytdl.svg)](https://crates.io/crates/shadowsocks)

a simple cli for downloading youtube video.

# Installation
```sh
cargo install ytdl
```
# Usage 
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
    -f, --filter <FILTER>...
            filter available formats, syntax: val1 val2 val3

    -o, --output <FILE>                 write output to a file
    -p, --proxy-url <PROXY_URL>         use proxy for the request
    -r, --range <RANGE>
            download a specific range of bytes of the video, [start]-[end]

        --start-offset <STARTOFFSET>    offset the start of the video

ARGS:
    <url>    youtube url
```

# Licenses

All source code is licensed under the [MIT License](https://github.com/cssivision/ytdl/blob/master/LICENSE).

