## 1. Project Goal

The primary goal is to implement all features as defined in the **"docs/architecture.md"** and **"docs/requirements.md"**. The application must be a robust, efficient, and user-friendly command-line tool.

## 2. Core Technology Stack

- **Language:** Rust (Latest Stable)
- **Asynchronous Runtime:** `tokio` shall be used for all asynchronous operations, including file I/O and network requests.

## 3. Recommended Crates (Libraries)

You are expected to build the application using the following high-quality crates. Justify any deviation from this list.

- **Configuration Parsing (`config.toml`):**
    - `toml`: For parsing the TOML file format.
    - `serde`: For deserializing the configuration data into a strongly-typed Rust struct.
- **Folder Monitoring:**
    - `notify`: For watching filesystem events in an asynchronous, cross-platform manner. Use `notify-debouncer-full` for more robust event handling.
- **File Hashing:**
    - `sha2`: To compute the SHA-256 hash of files. Implement this using a buffered reader to handle large files efficiently without consuming excessive memory.
- **Image Conversion (to AVIF):**
    - `image`: To decode source images (`png`, `jpeg`, etc.).
    - `ravif` or `cavif`: To encode the decoded image data into the AVIF format.
- **Cloudflare R2 API Client:**
    - `aws-sdk-s3`: Use the official AWS SDK for Rust to interact with R2's S3-compatible API.
    - `aws-config`: To load configuration and credentials for the SDK. The R2 endpoint URL must be configurable.
- **Clipboard Interaction:**
    - `arboard`: A cross-platform library for accessing the system clipboard.
- **Logging:**
    - `tracing` or `log`: Implement structured logging to provide clear feedback to the user about ongoing operations, successes, and failures. `env_logger` (for `log`) or `tracing-subscriber` (for `tracing`) can be used for simple setup.

## 4. Application Architecture

- **Modularity:** Structure the codebase into logical modules to separate concerns. A suggested structure:
    - `main.rs`: Entry point, application setup, and starting the main loop.
    - `config.rs`: Defines the `Config` struct and handles loading/validation.
    - `watcher.rs`: Manages folder monitoring and dispatches file events.
    - `handler.rs` or `processor.rs`: Contains the core logic for processing a single file (hashing, converting, uploading, etc.).
    - `r2.rs`: Encapsulates all interactions with Cloudflare R2 (upload, check existence).
    - `clipboard.rs`: A small module for clipboard operations.
    - `error.rs`: Defines the project-specific `Error` enum and `Result` type alias.

## 5. Testing Strategy

- **Unit Tests:** Place unit tests within each module (`#[cfg(test)]`). Focus on testing pure functions, such as URL generation, filename creation, and specific data transformations.
- **Integration Tests:** Create an integration test suite in the `tests/` directory.
    - These tests should cover the entire workflow.
    - Use temporary directories and mock files to test the file system interactions (`done` and `error` folder logic).
    - For R2 interactions, use a mocking library like `mockall` to simulate API responses without making actual network calls. Test duplicate handling and upload logic thoroughly.
