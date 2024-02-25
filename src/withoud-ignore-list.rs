use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

fn zip_current_directory(zip_file_path: &str) -> std::io::Result<()> {
    let file = File::create(zip_file_path)?;
    let mut zip = ZipWriter::new(file);

    // Recursively add files in the current directory to the zip file
    add_directory_to_zip(&mut zip, ".")?;

    zip.finish()?;
    Ok(())
}

fn add_directory_to_zip(
    zip: &mut ZipWriter<File>,
    path: &str,
) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Recursively add subdirectories
            add_directory_to_zip(zip, &path.to_string_lossy())?;
        } else {
            // Add file to the zip archive
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755); // Set file permissions if needed
            zip.start_file(path.to_string_lossy(), options)?;
            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }
    Ok(())
}

fn main() {
    let zip_file_path = "./archive.zip";
    if let Err(e) = zip_current_directory(zip_file_path) {
        eprintln!("Error zipping current directory: {}", e);
    } else {
        println!("Current directory zipped successfully.");
    }
}
