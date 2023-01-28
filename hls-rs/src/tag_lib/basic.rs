use crate::error_lib::TagError;
use std::io;

pub enum BasicTag {
    /// ## [EXTM3U](https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.1.1)
    ///
    /// The EXTM3U tag indicates that the file is an Extended M3U [M3U] Playlist file
    ///
    FileType,
    /// ## [EXT-X-VERSION](https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.1.2)
    ///
    /// The EXT-X-VERSION tag indicates the compatibility version of the Playlist file, its associated media, and its server.
    ///
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
