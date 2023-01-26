use thiserror::Error;

#[derive(Error, Debug)]
pub enum StreamUrlError {
    #[error("Invalid protocol use in uri MUST be http or https")]
    InvalidProtocol,
    #[error("Invalid ending extension HLS stream MUSt end in .m3u8")]
    InvalidEndingExtension,
}

#[derive(Error, Debug)]
pub enum PlaylistDownloadError {
    #[error("Request for playlist failed")]
    RequestFailed,
    #[error("Failed to read request data")]
    FailedToReadResponse,
    #[error("Failed to create temp file")]
    FailedToCreateTempFile,
    #[error("Failed to copy playlist to temp file")]
    FailedToCopyData,
}

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("Failed to open file")]
    FailedToOpen,
}
