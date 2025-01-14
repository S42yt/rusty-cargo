use std::process::{Command, exit};
use std::path::Path;
use walkdir::WalkDir;

fn format_file(file_path: &str) {
    let status = Command::new("rustfmt")
        .arg(file_path)
        .status()
        .expect("Failed to execute rustfmt");

    if !status.success() {
        eprintln!("Failed to format file: {}", file_path);
    } else {
        println!("Formatted: {}", file_path);
    }
}

fn format_all_files() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    for entry in WalkDir::new(current_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
    {
        let file_path = entry.path().to_str().expect("Invalid file path");
        format_file(file_path);
    }
}

fn main() {
    if !Command::new("rustfmt").output().is_ok() {
        eprintln!("rustfmt is not installed or not in the PATH.");
        exit(1);
    }

    println!("Starting to format Rust files...");

    format_all_files();

    println!("Rust file formatting completed!");
}
