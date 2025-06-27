## 1. Overview

### 1.1. Objective
The objective is to streamline the content-sharing workflow by monitoring a specific local folder, automatically processing and uploading any added image or video files to Cloudflare R2, and placing their public URLs onto the system clipboard.

### 1.2. Deliverables
* A command-line application written in Rust that meets the functional requirements outlined below.
* A template for the configuration file (`config.toml`).

## 2. System Overview

This system operates as a single executable file that monitors a local folder specified by the user. When a file is added, it automatically performs a series of operations (conversion, naming, uploading) based on the file type (image or video), outputs the result to the clipboard, and archives the original file.

### 2.1. Processing Flow

```
Start
│
├─ 1. Load configuration from config.toml
│
├─ 2. Begin monitoring the specified folder
│
└──<File Addition Event>
│
├─ 3. Calculate file hash (SHA-256)
│
├─ 4. [R2] Check if a file with the same hash exists in the bucket
│   │
│   ├─ (Yes) → 8. Delete the local source file and end process
│   │
│   └─ (No) → 5. Determine file type
│
├─ 5. Determine File Type
│   │
│   ├─ [Image] → 5a. Convert to AVIF
│   │
│   └─ [Video] → 5b. No conversion
│
├─ 6. Upload to R2 (Filename: {hash}.{extension})
│
├─ 7. Copy URL to clipboard
│   │
│   ├─ [Image] →
│   │
│   └─ [Video] → <video controls src="https://www.google.com/search?q=Public_URL"></video>
│
├─ 8. Move the original file to done/ (Rename: {hash}.{original_extension})
│
└── Process Complete (Waiting for the next event)
  ※ If an error occurs at any step:
      └─ 9. Log the error and move the original file to `error/`
```

## 3. Functional Requirements

### 3.1. Configuration
- The application is configured by a `config.toml` file located in the same directory as the executable.
- On startup, the application will read `config.toml`. If the configuration is invalid or incomplete, it will display an error message and exit.
- The configuration items are as follows:
  - `account_id`: Your Cloudflare Account ID.
  - `access_key_id`: Your R2 Access Key ID.
  - `secret_access_key`: Your R2 Secret Access Key.
  - `bucket_name`: The target R2 bucket name.
  - `public_url_base`: The base of the public URL for your bucket (e.g., `https://pub-xxxxxxxx.r2.dev` or a custom domain like `https://r2.example.com`). No trailing slash.
  - `watch_folder`: The absolute or relative path to the folder to be monitored.

### 3.2. Folder Monitoring
- Monitors the `watch_folder` specified in `config.toml`.
- Monitoring is limited to the top-level directory and does not include subdirectories.
- The process is triggered by a "file created" event in the folder.

### 3.3. Image Processing
- **Target Extensions:** `jpg`, `jpeg`, `png`, `gif`, `webp`, `bmp` (case-insensitive).
- **Filename Generation:** Calculates an SHA-256 hash from the file's content. This hash is used as the base for the filename.
- **Image Conversion:** Converts the target file to AVIF format. Only AVIF files will be uploaded to R2 for images.

### 3.4. Video Processing
- **Target Extension:** `mp4` (case-insensitive).
- **Filename Generation:** Same as for images, an SHA-256 hash is calculated from the file's content.
- **Conversion:** None. The original `mp4` file is uploaded as is.

### 3.5. Cloudflare R2 Upload
- Connects to Cloudflare R2 using the S3-compatible API.
- **Object Key:** The filename on R2 will be `{hash}.{extension}`.
  - For images: `{SHA256_hash}.avif`
  - For videos: `{SHA256_hash}.mp4`
- **Duplicate File Handling:**
  - Before uploading, the application checks if an object with the same key already exists in the bucket.
  - If it exists, the upload and all subsequent steps (clipboard copy, file move) are skipped, and the local source file is deleted.

### 3.6. Clipboard Integration
- After a successful upload, the public URL is written to the system clipboard.
- **For Images:** The URL is formatted as Markdown.
  - Format: `![]({public_url_base}/{hash}.avif)`
- **For Videos:** The URL is formatted as an HTML video tag. The `controls` attribute is added for better usability.
  - Format: `<video controls src="{public_url_base}/{hash}.mp4"></video>`

### 3.7. File Post-processing
- This step runs only after the upload and clipboard copy operations are completed successfully.
- An `done` folder is automatically created inside the `watch_folder` if it does not exist.
- The processed original file is moved to the `done` folder.
- When moved, the file is renamed to `{hash}.{original_extension}` to make it easier to identify the original file later (e.g., `original.png` becomes `done/d8e8fca2....png`).

### 3.8. Error Handling
- If any step in the process fails (hashing, conversion, upload, clipboard copy, file move), all subsequent steps are aborted.
- The error details are printed to standard error (the console).
- An `error` folder is automatically created inside the `watch_folder` if it does not exist.
- The original file that caused the error is moved to the `error` folder, and the process for that file terminates.

## 4. Non-Functional Requirements

### 4.1. Execution Environment
- **Platform:** A single, self-contained executable binary that runs on Windows, macOS, and Linux.
- **Dependencies:** Requires no external library or runtime installations by the user.

### 4.2. Performance
- File monitoring will be performed in real-time to minimize latency between file addition and process completion.
- The implementation should be memory-efficient to handle large video files (on the order of gigabytes).

### 4.3. Security
- Cloudflare R2 authentication credentials (in `config.toml`) are stored and used only on the local filesystem. They are never transmitted anywhere except for authenticating with the R2 API. The user is responsible for securing the configuration file via filesystem permissions.
