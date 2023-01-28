use crate::tag_lib::error::TagError;
use std::io;

pub trait TagType {
    fn as_str(&self) -> &'static str;

    fn from_line(line: Result<String, io::Error>) -> Result<Self, TagError>
    where
        Self: Sized;
}

pub fn parse_line(line: String) -> Result<String, TagError> {
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

enum MediaOrMasterPlaylistTags {
    /// ## [EXT-X-INDEPENDENT-SEGMENTS](https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.5.1)
    ///
    /// The EXT-X-INDEPENDENT-SEGMENTS tag indicates that all media samples
    /// in a Media Segment can be decoded without information from other
    /// segments.
    ///
    IndependentSegments,
    /// ## [EXT-X-START](https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.5.2)
    ///
    /// The EXT-X-START tag indicates a preferred point at which to start
    /// playing a Playlist.  By default, clients SHOULD start playback at
    /// this point when beginning a playback session.
    ///
    /// ### Attributes
    ///  * TIME-OFFSET
    ///  * PRECISE
    ///
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
