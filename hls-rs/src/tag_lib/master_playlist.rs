use crate::tag_lib::error::TagError;
use crate::tag_lib::shared::{parse_line, TagType};
use std::io;

pub enum MasterPlaylistTag {
    Rendition,
    VariantStream,
    IFrameStream,
    SessionData,
    SessionKey,
}

impl TagType for MasterPlaylistTag {
    fn as_str(&self) -> &'static str {
        match self {
            MasterPlaylistTag::Rendition => "EXT-X-MEDIA",
            MasterPlaylistTag::VariantStream => "EXT-X-STREAM-INF",
            MasterPlaylistTag::IFrameStream => "EXT-X-I-FRAME-STREAM-INF",
            MasterPlaylistTag::SessionData => "EXT-X-SESSION-DATA",
            MasterPlaylistTag::SessionKey => "EXT-X-SESSION-KEY",
        }
    }

    fn from_line(line: Result<String, io::Error>) -> Result<Self, TagError> {
        match line {
            Ok(line_str) => {
                if let Ok(tag_check) = parse_line(line_str) {
                    match tag_check.as_str() {
                        "EXT-X-MEDIA" => Ok(MasterPlaylistTag::Rendition),
                        "EXT-X-STREAM-INF" => Ok(MasterPlaylistTag::VariantStream),
                        "EXT-X-I-FRAME-STREAM-INF" => Ok(MasterPlaylistTag::IFrameStream),
                        "EXT-X-SESSION-DATA" => Ok(MasterPlaylistTag::SessionData),
                        "EXT-X-SESSION-KEY" => Ok(MasterPlaylistTag::SessionKey),
                        _ => Err(TagError::NoMatchingTags),
                    }
                } else {
                    Err(TagError::FailedToParse)
                }
            }
            Err(_e) => Err(TagError::FailedToParse),
        }
    }
}
