use std::collections::HashMap;

pub const FORMAT_EXTENSION_KEY: &str = "ext";
pub const FORMAT_rESOLUTION_KEY: &str = "res";
pub const FORMAT_VIDEO_ENCODING_KEY: &str = "videnc";
pub const FORMAT_AUDIO_ENCODING_KEY: &str = "audenc";
pub const FORMAT_iTAG_KEY: &str = "itag";
pub const FORMAT_AUDIO_BITRATE_KEY: &str = "audbr";


#[derive(Default, Clone)]
pub struct Format {
    pub itag: i32,
	pub audio_bitrate: i32,
	pub extension: String,
	pub resolution: String,
	pub video_encoding: String,
	pub audio_encoding: String,
	pub meta: HashMap<String, String>,
}

// FORMATS is a map of all itags and their formats
lazy_static! {
    static ref FORMATS: HashMap<i32, Format> = {
        let mut m = HashMap::new();
		let DATA = [
			(5, Format{
				extension:     "flv".to_string(),
				resolution:    "240p".to_string(),
				video_encoding: "Sorenson H.283".to_string(),
				audio_encoding: "mp3".to_string(),
				itag:          5,
				audio_bitrate:  64,
				..Default::default()
			}),
			(6, Format{
				extension:     "flv".to_string(),
				resolution:    "270p".to_string(),
				video_encoding: "Sorenson H.263".to_string(),
				audio_encoding: "mp3".to_string(),
				itag:          6,
				audio_bitrate:  64,
				..Default::default()
			}),
			(13, Format{
				extension:     "3gp".to_string(),
				resolution:    "".to_string(),
				video_encoding: "MPEG-4 Visual".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          13,
				audio_bitrate:  0,
				..Default::default()
			}),
			(17, Format{
				extension:     "3gp".to_string(),
				resolution:    "144p".to_string(),
				video_encoding: "MPEG-4 Visual".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          17,
				audio_bitrate:  24,
				..Default::default()
			}),
			(18, Format{
				extension:     "mp4".to_string(),
				resolution:    "360p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          18,
				audio_bitrate:  96,
				..Default::default()
			}),
			(22, Format{
				extension:     "mp4".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          22,
				audio_bitrate:  192,
				..Default::default()
			}),
			(34, Format{
				extension:     "flv".to_string(),
				resolution:    "480p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          34,
				audio_bitrate:  128,
				..Default::default()
			}),
			(35, Format{
				extension:     "flv".to_string(),
				resolution:    "360p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          35,
				audio_bitrate:  128,
				..Default::default()
			}),
			(36, Format{
				extension:     "3gp".to_string(),
				resolution:    "240p".to_string(),
				video_encoding: "MPEG-4 Visual".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          36,
				audio_bitrate:  36,
				..Default::default()
			}),
			(37, Format{
				extension:     "mp4".to_string(),
				resolution:    "1080p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          37,
				audio_bitrate:  192,
				..Default::default()
			}),
			(38, Format{
				extension:     "mp4".to_string(),
				resolution:    "3072p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          38,
				audio_bitrate:  192,
				..Default::default()
			}),
			(43, Format{
				extension:     "webm".to_string(),
				resolution:    "360p".to_string(),
				video_encoding: "VP8".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          43,
				audio_bitrate:  128,
				..Default::default()
			}),
			(44, Format{
				extension:     "webm".to_string(),
				resolution:    "480p".to_string(),
				video_encoding: "VP8".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          44,
				audio_bitrate:  128,
				..Default::default()
			}),
			(45, Format{
				extension:     "webm".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "VP8".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          45,
				audio_bitrate:  192,
				..Default::default()
			}),
			(46, Format{
				extension:     "webm".to_string(),
				resolution:    "1080p".to_string(),
				video_encoding: "VP8".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          46,
				audio_bitrate:  192,
				..Default::default()
			}),
			(82, Format{
				extension:     "mp4".to_string(),
				resolution:    "360p".to_string(),
				video_encoding: "H.264".to_string(),
				itag:          82,
				audio_bitrate:  96,
				..Default::default()
			}),
			(83, Format{
				extension:     "mp4".to_string(),
				resolution:    "240p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          83,
				audio_bitrate:  96,
				..Default::default()
			}),
			(84, Format{
				extension:     "mp4".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          84,
				audio_bitrate:  192,
				..Default::default()
			}),
			(85, Format{
				extension:     "mp4".to_string(),
				resolution:    "1080p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          85,
				audio_bitrate:  192,
				..Default::default()
			}),
			(100, Format{
				extension:     "webm".to_string(),
				resolution:    "360p".to_string(),
				video_encoding: "VP8".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          100,
				audio_bitrate:  128,
				..Default::default()
			}),
			(101, Format{
				extension:     "webm".to_string(),
				resolution:    "360p".to_string(),
				video_encoding: "VP8".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          101,
				audio_bitrate:  192,
				..Default::default()
			}),
			(102, Format{
				extension:     "webm".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "VP8".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          102,
				audio_bitrate:  192,
				..Default::default()
			}),
			// DASH (video only)
			(133, Format{
				extension:     "mp4".to_string(),
				resolution:    "240p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          133,
				audio_bitrate:  0,
				..Default::default()
			}),
			(134, Format{
				extension:     "mp4".to_string(),
				resolution:    "360p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          134,
				audio_bitrate:  0,
				..Default::default()
			}),
			(135, Format{
				extension:     "mp4".to_string(),
				resolution:    "480p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          135,
				audio_bitrate:  0,
				..Default::default()
			}),
			(136, Format{
				extension:     "mp4".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          136,
				audio_bitrate:  0,
				..Default::default()
			}),
			(137, Format{
				extension:     "mp4".to_string(),
				resolution:    "1080p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          137,
				audio_bitrate:  0,
				..Default::default()
			}),
			(138, Format{
				extension:     "mp4".to_string(),
				resolution:    "2160p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          138,
				audio_bitrate:  0,
				..Default::default()
			}),
			(160, Format{
				extension:     "mp4".to_string(),
				resolution:    "144p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          160,
				audio_bitrate:  0,
				..Default::default()
			}),
			(242, Format{
				extension:     "webm".to_string(),
				resolution:    "240p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          242,
				audio_bitrate:  0,
				..Default::default()
			}),
			(243, Format{
				extension:     "webm".to_string(),
				resolution:    "360p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          243,
				audio_bitrate:  0,
				..Default::default()
			}),
			(244, Format{
				extension:     "webm".to_string(),
				resolution:    "480p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          244,
				audio_bitrate:  0,
				..Default::default()
			}),
			(247, Format{
				extension:     "webm".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          247,
				audio_bitrate:  0,
				..Default::default()
			}),
			(248, Format{
				extension:     "webm".to_string(),
				resolution:    "1080p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          248,
				audio_bitrate:  9,
				..Default::default()
			}),
			(264, Format{
				extension:     "mp4".to_string(),
				resolution:    "1440p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          264,
				audio_bitrate:  0,
				..Default::default()
			}),
			(266, Format{
				extension:     "mp4".to_string(),
				resolution:    "2160p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          266,
				audio_bitrate:  0,
				..Default::default()
			}),
			(271, Format{
				extension:     "webm".to_string(),
				resolution:    "1440p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          271,
				audio_bitrate:  0,
				..Default::default()
			}),
			(272, Format{
				extension:     "webm".to_string(),
				resolution:    "2160p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          272,
				audio_bitrate:  0,
				..Default::default()
			}),
			(278, Format{
				extension:     "webm".to_string(),
				resolution:    "144p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          278,
				audio_bitrate:  0,
				..Default::default()
			}),
			(298, Format{
				extension:     "mp4".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          298,
				audio_bitrate:  0,
				..Default::default()
			}),
			(299, Format{
				extension:     "mp4".to_string(),
				resolution:    "1080p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "".to_string(),
				itag:          299,
				audio_bitrate:  0,
				..Default::default()
			}),
			(302, Format{
				extension:     "webm".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          302,
				audio_bitrate:  0,
				..Default::default()
			}),
			(303, Format{
				extension:     "webm".to_string(),
				resolution:    "1080p".to_string(),
				video_encoding: "VP9".to_string(),
				audio_encoding: "".to_string(),
				itag:          303,
				audio_bitrate:  0,
				..Default::default()
			}),
			// DASH (audio only)
			(139, Format{
				extension:     "mp4".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          139,
				audio_bitrate:  48,
				..Default::default()
			}),
			(140, Format{
				extension:     "mp4".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          140,
				audio_bitrate:  128,
				..Default::default()
			}),
			(141, Format{
				extension:     "mp4".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          141,
				audio_bitrate:  256,
				..Default::default()
			}),
			(171, Format{
				extension:     "webm".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          171,
				audio_bitrate:  128,
				..Default::default()
			}),
			(172, Format{
				extension:     "webm".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "vorbis".to_string(),
				itag:          172,
				audio_bitrate:  192,
				..Default::default()
			}),
			(249, Format{
				extension:     "webm".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "opus".to_string(),
				itag:          249,
				audio_bitrate:  50,
				..Default::default()
			}),
			(250, Format{
				extension:     "webm".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "opus".to_string(),
				itag:          250,
				audio_bitrate:  70,
				..Default::default()
			}),
			(251, Format{
				extension:     "webm".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "opus".to_string(),
				itag:          251,
				audio_bitrate:  160,
				..Default::default()
			}),
			// Live streaming
			(92, Format{
				extension:     "ts".to_string(),
				resolution:    "240p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          92,
				audio_bitrate:  48,
				..Default::default()
			}),
			(93, Format{
				extension:     "ts".to_string(),
				resolution:    "480p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          93,
				audio_bitrate:  128,
				..Default::default()
			}),
			(94, Format{
				extension:     "ts".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          94,
				audio_bitrate:  128,
				..Default::default()
			}),
			(95, Format{
				extension:     "ts".to_string(),
				resolution:    "1080p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          95,
				audio_bitrate:  256,
				..Default::default()
			}),
			(96, Format{
				extension:     "ts".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          96,
				audio_bitrate:  256,
				..Default::default()
			}),
			(120, Format{
				extension:     "flv".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          120,
				audio_bitrate:  128,
				..Default::default()
			}),
			(127, Format{
				extension:     "ts".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          127,
				audio_bitrate:  96,
				..Default::default()
			}),
			(128, Format{
				extension:     "ts".to_string(),
				resolution:    "".to_string(),
				video_encoding: "".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          128,
				audio_bitrate:  96,
				..Default::default()
			}),
			(132, Format{
				extension:     "ts".to_string(),
				resolution:    "240p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          132,
				audio_bitrate:  48,
				..Default::default()
			}),
			(151, Format{
				extension:     "ts".to_string(),
				resolution:    "720p".to_string(),
				video_encoding: "H.264".to_string(),
				audio_encoding: "aac".to_string(),
				itag:          151,
				audio_bitrate:  24,
				..Default::default()
			}),
		];

		for v in DATA.iter() {
			m.insert(v.0, v.1.clone());
		}
		m 
	};
}

impl Format {
	fn new(itag: i32) -> Option<Format> {
		FORMATS.get(&itag).map(|f| f.clone())
	}
}
