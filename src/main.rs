use std::env;
use std::fs;
use std::process::{exit, Command};
use walkdir::WalkDir;

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
        println!("Formatted: {}", file_path);
        return true;
    }

    false
}

fn format_all_files() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

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
    if args.len() > 1 && args[1] == "rusty" {
        if !Command::new("rustfmt").output().is_ok() {
            eprintln!("rustfmt is not installed or not in the PATH.");
            exit(1);
        }
        println!("Formatting files...");
        format_all_files();
        println!("Done!");
        return;
    }
}