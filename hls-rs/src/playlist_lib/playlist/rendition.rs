enum RenditionType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}

impl RenditionType {
    fn as_str(&self) -> &'static str {
        match self {
            RenditionType::Audio => "AUDIO",
            RenditionType::Video => "Video",
            RenditionType::Subtitles => "SUBTITLES",
            RenditionType::ClosedCaptions => "CLOSED-CAPTIONS",
        }
    }
}

enum RenditionYesNo {
    Yes,
    No,
}

impl RenditionYesNo {
    fn as_str(&self) -> &'static str {
        match self {
            RenditionYesNo::Yes => "YES",
            RenditionYesNo::No => "NO",
        }
    }
}

pub struct Rendition {
    rendition_type: RenditionType,
    uri: String,
    group_id: String,
    language: Option<String>,
    associated_language: Option<String>,
    name: String,
    default: Option<RenditionYesNo>,
    autoselect: Option<RenditionYesNo>,
    forced: Option<RenditionYesNo>,
    instream_id: Option<String>,
    characteristics: Option<Vec<String>>,
    channels: Option<String>,
}
