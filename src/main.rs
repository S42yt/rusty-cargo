use std::env;
use std::fs;
use std::process::{exit, Command};
use walkdir::WalkDir;

const VERSION: &str = "0.1.7";

fn format_file(file_path: &str) -> bool {
    let original_content = fs::read_to_string(file_path).expect("Failed to read file");

    let status = Command::new("rustfmt")
        .arg(file_path)
        .status()
        .expect("Failed to execute rustfmt");

    if !status.success() {
        eprintln!("Failed to format file: {}", file_path);
        return false;
    }

    let formatted_content = fs::read_to_string(file_path).expect("Failed to read formatted file");

    if original_content != formatted_content {
        let display_path = file_path.strip_prefix("src/").unwrap_or(file_path);
        println!("Formatted: {}", display_path);
        true
    } else {
        false
    }
}

fn format_all_files() {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    for entry in WalkDir::new(&current_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
    {
        let abs_path = entry.path().canonicalize().expect("Invalid path");
        let abs_str = abs_path.to_str().expect("Invalid file path");

        format_file(abs_str);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("Rusty Formatter v{}", VERSION);
        println!("Usage: rusty [OPTIONS]");
        println!("\nOptions:");
        println!("  -h, --help    Print help information");
        println!("  -v, --version Print version information");
        exit(0);
    }

    if args.contains(&"--version".to_string()) || args.contains(&"-v".to_string()) {
        println!("Rusty Formatter v{}", VERSION);
        exit(0);
    }

    if Command::new("rustfmt").output().is_err() {
        eprintln!("rustfmt is not installed or not in the PATH.");
        exit(1);
    }

    println!("Starting Rusty Formatter...");
    format_all_files();
    println!("Formatting completed!");
}
