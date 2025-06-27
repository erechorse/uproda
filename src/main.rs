use clap::Parser;
use std::path::Path;
use futures::future::join_all;
use std::error::Error;

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
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("No files provided for upload.");
        return Ok(());
    }

    let mut tasks = vec![];
    for file_path_str in args.files {
        let file_path = Path::new(&file_path_str).to_owned();
        let url = args.url.clone();
        tasks.push(tokio::spawn(async move {
            let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
            match uploader::upload_file(&file_path, &url).await {
                Ok(_) => println!("Finished uploading {}", file_name),
                Err(e) => eprintln!("Failed to upload {}: {}", file_name, e),
            }
            Ok::<(), Box<dyn Error + Send + Sync>>(()) // Return a Result from the spawned task
        }));
    }

    let results = join_all(tasks).await;

    for result in results {
        if let Err(e) = result {
            eprintln!("Task failed: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }

    #[tokio::test]
    async fn test_parallel_uploads() -> Result<(), Box<dyn Error + Send + Sync>> {
        let num_files = 3;
        let mut temp_files = Vec::new();
        let mut file_paths = Vec::new();

        for i in 0..num_files {
            let mut temp_file = NamedTempFile::new()?;
            temp_file.write_all(format!("This is test file {}.", i).as_bytes())?;
            file_paths.push(temp_file.path().to_owned());
            temp_files.push(temp_file); // Keep temp_file in scope
        }

        let url = "https://httpbin.org/post";

        let mut tasks = vec![];
        for file_path in file_paths {
            let url_clone = url.to_string();
            tasks.push(tokio::spawn(async move {
                uploader::upload_file(&file_path, &url_clone).await
            }));
        }

        let results = join_all(tasks).await;

        for result in results {
            assert!(result.is_ok()); // Check if the task itself completed without panicking
            assert!(result.unwrap().is_ok()); // Check if the upload was successful
        }

        Ok(())
    }
}
