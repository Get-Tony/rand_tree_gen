use rand::Rng;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

// Created by: Anthony Pagan

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(length);
    for _ in 0..length {
        let c = rng.gen_range(b'a'..=b'z');
        string.push(c as char);
    }
    string
}

fn validate_size(size: &str) -> Result<u64, String> {
    if size.ends_with('G') {
        size[..size.len() - 1]
            .parse::<u64>()
            .map_err(|_| "Invalid size format. Use G for Gigabytes or T for Terabytes.".to_string())
            .map(|n| n * 1024 * 1024 * 1024)
    } else if size.ends_with('T') {
        size[..size.len() - 1]
            .parse::<u64>()
            .map_err(|_| "Invalid size format. Use G for Gigabytes or T for Terabytes.".to_string())
            .map(|n| n * 1024 * 1024 * 1024 * 1024)
    } else {
        Err("Invalid size format. Use G for Gigabytes or T for Terabytes.".to_string())
    }
}

fn parse_arguments() -> Result<(u64, PathBuf), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Usage: gen_random_tree <size> <path>".to_string());
    }
    let size = validate_size(&args[1])?;
    let path = PathBuf::from(&args[2]);
    if path.exists() {
        if path.is_file() {
            return Err("Target path already exists as a file.".to_string());
        } else if !path.is_dir() {
            return Err("Target path is not a directory.".to_string());
        }
    }
    Ok((size, path))
}

fn create_structure(path: &PathBuf, size: u64) -> Result<(), io::Error> {
    if size < 100 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Target size is too small to create a directory tree.",
        ));
    }
    let mut current_size = 0;
    println!("Creating directory tree...");
    while current_size < size {
        let sub_path = path.join(format!("dir_{}", rand::thread_rng().gen_range(1..=1000)));
        fs::create_dir_all(&sub_path)?;
        let file_size = rand::thread_rng().gen_range(50..=200) * 1024 * 1024;
        let file_path = sub_path.join(format!(
            "file_{}.txt",
            rand::thread_rng().gen_range(1..=1000)
        ));
        let mut file = File::create(&file_path)?;
        let text = generate_random_string(file_size);
        file.write_all(text.as_bytes())?;
        current_size += file_size as u64; // convert file_size to u64
        println!(
            "Created file {} ({:.2}%)",
            file_path.display(),
            current_size as f64 / size as f64 * 100.0
        );
    }
    println!("Directory tree creation complete.");
    Ok(())
}

fn main() -> Result<(), String> {
    let (size, path) = parse_arguments()?;
    create_structure(&path, size).map_err(|e| e.to_string())
}
