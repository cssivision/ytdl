use std::collections::HashMap;

pub const FORMAT_EXTENSION_KEY: &str = "ext";
pub const FORMAT_RESOLUTION_KEY: &str = "res";
pub const FORMAT_VIDEO_ENCODING_KEY: &str = "videnc";
pub const FORMAT_AUDIO_ENCODING_KEY: &str = "audenc";
pub const FORMAT_ITAG_KEY: &str = "itag";
pub const FORMAT_AUDIO_BITRATE_KEY: &str = "audbr";

pub struct Format<'a> {
    itag: i32,
	extension: &'a str,
	resolution: &'a str,
	VideoEncoding: &'a str,
	AudioEncoding: &'a str,
	AudioBitrate: i32,
	meta: HashMap<&'a str, &'a str>,
}