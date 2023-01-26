use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use crate::error_lib::ReaderError;
use crate::stream_url::StreamUrl;

pub struct StreamReader {
    file_path: PathBuf,
}

impl StreamReader {
    pub fn new(file_path: PathBuf) -> StreamReader {
        StreamReader { file_path }
    }

    pub async fn from_url(url: String) -> Result<StreamReader, Box<dyn Error>> {
        let stream_url = StreamUrl::new(url);
        if let Err(e) = stream_url.is_valid() {
            return Err(Box::new(e));
        }

        let file_download_result = stream_url.download_to_file(None).await;

        match file_download_result {
            Ok(file_path) => Ok(StreamReader { file_path }),
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }

    pub fn to_lines(&self) -> Result<std::io::Lines<BufReader<File>>, ReaderError> {
        let file_result = File::open(&self.file_path);

        let Ok(file) = file_result else {
            return Err(ReaderError::FailedToOpen)
        };

        let reader = BufReader::new(file);
        let file_lines = reader.lines();

        Ok(file_lines)
    }
}
