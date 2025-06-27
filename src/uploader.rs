use reqwest::{Client, multipart};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use std::path::Path;

pub async fn upload_file(file_path: &Path, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let file_name = file_path.file_name().ok_or("Invalid file name")?.to_str().ok_or("Invalid file name")?;

    let file = File::open(file_path).await?;
    let file_size = file.metadata().await?.len();

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);

    let part = multipart::Part::stream_with_length(body, file_size)
        .file_name(file_name.to_string())
        .mime_str("application/octet-stream")?;

    let form = multipart::Form::new().part("file", part);

    let res = client.post(url)
        .multipart(form)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Successfully uploaded {} to {}", file_name, url);
        Ok(())
    } else {
        let status = res.status();
        let text = res.text().await?;
        println!("Upload failed for {}: Status: {}, Response: {}", file_name, status, text);
        Err(format!("Failed to upload {}: Status: {}, Response: {}", file_name, status, text).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_upload_file() -> Result<(), Box<dyn std::error::Error>> {
        // Create a dummy file for testing
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(b"This is a test file content.")?;
        let file_path = temp_file.path().to_owned();

        // Use httpbin.org/post for testing uploads
        let url = "https://httpbin.org/post";

        // Perform the upload
        let result = upload_file(&file_path, url).await;

        // Assert that the upload was successful
        assert!(result.is_ok(), "Upload failed with error: {:?}", result.err());

        Ok(())
    }
}