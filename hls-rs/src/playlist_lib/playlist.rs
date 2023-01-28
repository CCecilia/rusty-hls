use self::variant::Variant;

pub mod master_playlist;
pub mod rendition;
pub mod session_data;
pub mod variant;

struct MediaOrMasterPlaylistTagData {
    independent_segments: Option<bool>,
    start: Option<PlaylistStart>,
}

struct BasicTagData {
    url: String,
    version: u32,
}

enum PlaylistType {
    Event,
    Vod,
}

impl PlaylistType {
    fn as_str(&self) -> &'static str {
        match self {
            PlaylistType::Event => "EVENT",
            PlaylistType::Vod => "VOD",
        }
    }
}

enum KeyMethod {
    None,
    AES128,
    SampleAES,
}

impl KeyMethod {
    fn as_str(&self) -> &'static str {
        match self {
            KeyMethod::None => "NONE",
            KeyMethod::AES128 => "AES-128",
            KeyMethod::SampleAES => "SAMPLE-AES",
        }
    }
}

pub struct SegmentKey {
    method: KeyMethod,
    uri: Option<String>,
    init_vector: Option<Vec<u32>>,
    key_format: Option<String>,
}

pub struct MediaSegment {
    uri: String,
    duration: u32,
    byte_range_start: Option<u32>,
    byte_range_end: Option<u32>,
}

enum PrecisionStart {
    Yes,
    No,
}

impl PrecisionStart {
    fn as_str(&self) -> &'static str {
        match self {
            PrecisionStart::Yes => "YES",
            PrecisionStart::No => "NO",
        }
    }
}

pub struct PlaylistStart {
    time_offset: f64,
    precise: Option<PrecisionStart>,
}
