use clap::Parser;
use hls_stream_reader::StreamUrl;
use std::{path::PathBuf, process::exit};

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
    let hls_file_path: PathBuf;

    if let Some(url) = cli_args.stream_url {
        let stream_url = StreamUrl::new(url);
        if let Err(e) = stream_url.is_valid() {
            eprint!("{}", e);
            exit(1);
        }

        let file_download = stream_url.download_to_file(None).await;

        if let Ok(file_download) = file_download {
            hls_file_path = file_download;
        }
    } else if let Some(file_path) = cli_args.master_file {
        hls_file_path = PathBuf::from(file_path);
    }
}
