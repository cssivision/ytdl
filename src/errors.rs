use std::str;
use std::io;
use url;
use reqwest as request;

error_chain! {
    foreign_links {
        Utf8Error(str::Utf8Error);
        UrlParseError(url::ParseError);
        RequestError(request::Error);
        IoError(io::Error);
    }
}