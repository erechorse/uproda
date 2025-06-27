use clap::Parser;
use std::path::Path;

mod uploader;

/// A simple CLI to upload multiple files concurrently.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the file(s) to upload.
    #[arg(required = true)]
    files: Vec<String>,

    /// The target server URL.
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // For now, we only upload the first file.
    // In a later step, we will handle multiple files concurrently.
    if let Some(file_path_str) = args.files.first() {
        let file_path = Path::new(file_path_str);
        uploader::upload_file(file_path, &args.url).await?;
    } else {
        eprintln!("No files provided for upload.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}