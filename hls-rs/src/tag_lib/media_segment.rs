use crate::tag_lib::error::TagError;

use crate::tag_lib::shared::{parse_line, TagType};
use std::io;

pub enum MediaSegmentTag {
    Duration,
    ByteRange,
    Discontinuity,
    EncryptionKey,
    MediaInitialization,
    ProgramDateTime,
    DateRange,
}

impl TagType for MediaSegmentTag {
    fn as_str(&self) -> &'static str {
        match self {
            MediaSegmentTag::Duration => "EXTINF",
            MediaSegmentTag::ByteRange => "EXT-X-BYTERANGE",
            MediaSegmentTag::Discontinuity => "EXT-X-DISCONTINUITY",
            MediaSegmentTag::EncryptionKey => "EXT-X-KEY",
            MediaSegmentTag::MediaInitialization => "EXT-X-MAP",
            MediaSegmentTag::ProgramDateTime => "EXT-X-PROGRAM-DATE-TIME",
            MediaSegmentTag::DateRange => "EXT-X-DATERANGE",
        }
    }

    fn from_line(line: Result<String, io::Error>) -> Result<Self, TagError> {
        match line {
            Ok(line_str) => {
                if let Ok(tag_check) = parse_line(line_str) {
                    match tag_check.as_str() {
                        "EXTINF" => Ok(MediaSegmentTag::Duration),
                        "EXT-X-BYTERANGE" => Ok(MediaSegmentTag::ByteRange),
                        "EXT-X-DISCONTINUITY" => Ok(MediaSegmentTag::Discontinuity),
                        "EXT-X-KEY" => Ok(MediaSegmentTag::EncryptionKey),
                        "EXT-X-MAP" => Ok(MediaSegmentTag::MediaInitialization),
                        "EXT-X-PROGRAM-DATE-TIME" => Ok(MediaSegmentTag::ProgramDateTime),
                        "EXT-X-DATERANGE" => Ok(MediaSegmentTag::DateRange),
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
