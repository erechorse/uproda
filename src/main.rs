use clap::Parser;

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

fn main() {
    let args = Args::parse();

    println!("Files to upload: {:?}", args.files);
    println!("Target URL: {}", args.url);
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
