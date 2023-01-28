use thiserror::Error;

#[derive(Error, Debug)]
pub enum TagError {
    #[error("Failed to parse line into tag")]
    FailedToParse,
    #[error("No matching tags")]
    NoMatchingTags,
    #[error("Not a tag line")]
    NotATagLine,
}
