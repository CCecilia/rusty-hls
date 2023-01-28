pub enum HDCPLevel {
    None,
    Type1,
    Type0,
}

impl HDCPLevel {
    fn as_str(&self) -> &'static str {
        match self {
            HDCPLevel::None => "NONE",
            HDCPLevel::Type1 => "TYPE-1",
            HDCPLevel::Type0 => "TYPE-0",
        }
    }
}

pub struct Variant {
    bandwidth: u32,
    average_bandwidth: Option<u32>,
    codecs: Vec<String>,
    resolution: Option<(u32, u32)>,
    frame_rate: Option<f64>,
    hdcp_level: Option<HDCPLevel>,
    audio: Option<String>,
    video: Option<String>,
    subtitles: Option<String>,
    closed_captions: Option<String>,
    uri: String,
    is_iframe: Option<bool>,
    is_rendition: bool,
}
