use std::{env::temp_dir, error::Error, fmt, fs, io, path::PathBuf};

#[derive(Debug)]
pub enum StreamUrlError {
    InvalidProtocol,
    InvalidEndingExtension,
}

impl Error for StreamUrlError {}

impl fmt::Display for StreamUrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamUrlError::InvalidProtocol => {
                write!(f, "Invalid protocol use in uri MUST be http or https")
            }
            StreamUrlError::InvalidEndingExtension => {
                write!(f, "Invalid ending extension HLS stream MUSt end in .m3u8")
            }
        }
    }
}

#[derive(Debug)]
pub enum PlaylistDownloadError {
    RequestFailed,
    FailedToReadResponse,
    FailedToCreateTempFile,
    FailedToCopyData,
}

impl Error for PlaylistDownloadError {}

impl fmt::Display for PlaylistDownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlaylistDownloadError::RequestFailed => write!(f, "request for playlist failed"),
            PlaylistDownloadError::FailedToReadResponse => write!(f, "Failed to read request data"),
            PlaylistDownloadError::FailedToCreateTempFile => {
                write!(f, "Failed to create temp file")
            }
            PlaylistDownloadError::FailedToCopyData => {
                write!(f, "Failed to copy playlist to temp file")
            }
        }
    }
}

pub struct StreamUrl {
    pub uri: String,
}

impl StreamUrl {
    pub fn new(uri: String) -> Self {
        StreamUrl { uri }
    }

    pub fn is_valid(&self) -> Result<(), StreamUrlError> {
        if !self.uri.starts_with("http") && !self.uri.starts_with("https") {
            return Err(StreamUrlError::InvalidProtocol);
        }

        if !self.uri.ends_with(".m3u8") {
            return Err(StreamUrlError::InvalidEndingExtension);
        }

        Ok(())
    }

    pub async fn download_to_file(
        &self,
        specific_file_path: Option<PathBuf>,
    ) -> Result<PathBuf, PlaylistDownloadError> {
        let file_path;

        if let Some(specific_file_path) = specific_file_path {
            file_path = specific_file_path;
        } else {
            let temp_dir = temp_dir();
            file_path = temp_dir.join("downloaded_playlist.m3u8");
        }

        let temp_file_result = fs::File::create(&file_path);

        let Ok(response) = reqwest::get(&self.uri).await else {
            return Err(PlaylistDownloadError::RequestFailed);
        };

        let Ok(response_data) = response.text().await else {
            return Err(PlaylistDownloadError::FailedToReadResponse);
        };

        let Ok(mut temp_file) = temp_file_result else {
            return Err(PlaylistDownloadError::FailedToCreateTempFile);
        };

        if io::copy(&mut response_data.as_bytes(), &mut temp_file).is_err() {
            return Err(PlaylistDownloadError::FailedToCopyData);
        };

        Ok(file_path)
    }
}
