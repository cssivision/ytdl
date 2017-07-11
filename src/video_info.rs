use std::error::Error;
use std::collections::HashMap;
use url::{Url};

pub fn get_video_info(value: &str) -> Result<i32, Box<Error>> {
    let parse_url = match Url::parse(value) {
        Ok(u) => u,
        Err(_) => {
            return get_info_from_id(value);
        },
    };

    let query: HashMap<String, String> = parse_url.query_pairs().into_owned().collect();
}

fn get_info_from_url() {

}

fn get_info_from_id(id: &str) {

}