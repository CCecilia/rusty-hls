use std::path::PathBuf;

pub struct StreamReader {
    file_path: PathBuf,
}

impl StreamReader {
    pub fn new(file_path: PathBuf) -> StreamReader {
        StreamReader { file_path }
    }
}
