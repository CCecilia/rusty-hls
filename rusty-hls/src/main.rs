use clap::Parser;
use hls_stream_reader::reader_lib::StreamReader;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "HLS Parser")]
#[command(author = "Christian Cecilia <christian.cecilia1@gmail.com@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Parses an HLS stream for all relevant data and serializes tags", long_about = None)]
struct CLI {
    #[arg(long, help = "Stream url to fetch manifest from.")]
    stream_url: Option<String>,
    #[arg(long, help = "Stream url to fetch manifest from.")]
    master_file: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli_args = CLI::parse();
    let mut hls_file_path: Option<PathBuf> = None;

    if let Some(url) = cli_args.stream_url {
        let reader = StreamReader::from_url(url).await;
    }

    if let Some(file_path) = cli_args.master_file {
        let reader = StreamReader::new(PathBuf::from(file_path));
    }
}
