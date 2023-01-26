use crate::error_lib::TagError;
use std::io;

pub trait TagType {
    fn as_str(&self) -> &'static str;

    fn from_line(line: Result<String, io::Error>) -> Result<Self, TagError>
    where
        Self: Sized;
}

fn parse_line(line: String) -> Result<String, TagError> {
    if !line.starts_with("#") {
        return Err(TagError::NotATagLine);
    }

    let tag_delim = ":";

    if line.contains(tag_delim) {
        if let Some((tag, _attrs)) = line.split_once(tag_delim) {
            let fmt_tag = tag.replace("#", "");
            return Ok(fmt_tag);
        };
    } else {
        let fmt_tag = line.replace("#", "");
        return Ok(fmt_tag);
    }

    Err(TagError::FailedToParse)
}

pub enum BasicTag {
    FileType,
    Version,
}

impl TagType for BasicTag {
    fn as_str(&self) -> &'static str {
        match self {
            BasicTag::FileType => "EXTM3U",
            BasicTag::Version => "EXT-X-VERSION",
        }
    }

    fn from_line(line: Result<String, io::Error>) -> Result<Self, TagError> {
        match line {
            Ok(line_str) => {
                if let Ok(tag_check) = parse_line(line_str) {
                    match tag_check.as_str() {
                        "EXTM3U" => Ok(BasicTag::FileType),
                        "EXT-X-VERSION" => Ok(BasicTag::Version),
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

pub enum MediaPlaylistTag {
    TargetDduration,
    MediaSequence,
    DiscontinuitySequence,
    End,
    Type,
    IFramesOnly,
}

impl TagType for MediaPlaylistTag {
    fn as_str(&self) -> &'static str {
        match self {
            MediaPlaylistTag::TargetDduration => "EXT-X-TARGETDURATION",
            MediaPlaylistTag::MediaSequence => "EXT-X-MEDIA-SEQUENCE",
            MediaPlaylistTag::DiscontinuitySequence => "EXT-X-DISCONTINUITY-SEQUENCE",
            MediaPlaylistTag::End => "EXT-X-ENDLIST",
            MediaPlaylistTag::Type => "EXT-X-PLAYLIST-TYPE",
            MediaPlaylistTag::IFramesOnly => "EXT-X-I-FRAMES-ONLY",
        }
    }

    fn from_line(line: Result<String, io::Error>) -> Result<Self, TagError> {
        match line {
            Ok(line_str) => {
                if let Ok(tag_check) = parse_line(line_str) {
                    match tag_check.as_str() {
                        "EXT-X-TARGETDURATION" => Ok(MediaPlaylistTag::TargetDduration),
                        "EXT-X-MEDIA-SEQUENCE" => Ok(MediaPlaylistTag::MediaSequence),
                        "EXT-X-DISCONTINUITY-SEQUENCE" => {
                            Ok(MediaPlaylistTag::DiscontinuitySequence)
                        }
                        "EXT-X-ENDLIST" => Ok(MediaPlaylistTag::End),
                        "EXT-X-PLAYLIST-TYPE" => Ok(MediaPlaylistTag::Type),
                        "EXT-X-I-FRAMES-ONLY" => Ok(MediaPlaylistTag::IFramesOnly),
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

enum MediaOrMasterPlaylistTags {
    IndependentSegments,
    Start,
}

impl TagType for MediaOrMasterPlaylistTags {
    fn as_str(&self) -> &'static str {
        match self {
            MediaOrMasterPlaylistTags::IndependentSegments => "EXT-X-INDEPENDENT-SEGMENTS",
            MediaOrMasterPlaylistTags::Start => "EXT-X-START",
        }
    }

    fn from_line(line: Result<String, io::Error>) -> Result<Self, TagError> {
        match line {
            Ok(line_str) => {
                if let Ok(tag_check) = parse_line(line_str) {
                    match tag_check.as_str() {
                        "EXT-X-INDEPENDENT-SEGMENTS" => {
                            Ok(MediaOrMasterPlaylistTags::IndependentSegments)
                        }
                        "EXT-X-START" => Ok(MediaOrMasterPlaylistTags::Start),
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
