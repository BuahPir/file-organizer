use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "File Organizer")]
#[command(about = "Organize files by their extensions into subdirectories", long_about = None)]
struct Args {
    #[arg(value_name = "PATH")]
    directory: PathBuf,

    #[arg(short, long)]
    by_extension: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.directory.exists() {
        anyhow::bail!("Directory does not exist: {:?}", args.directory);
    }

    if !args.directory.is_dir() {
        anyhow::bail!("Path is not a directory: {:?}", args.directory);
    }

    organize_files(&args.directory)
        .context("Failed to organize files")?;

    Ok(())
}

fn organize_files(dir: &Path) -> Result<()> {
    let entries = fs::read_dir(dir).context("Failed to read directory")?;

    let mut organized_count = 0;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        // It is a directory, skip
        if path.is_dir() {
            continue;
        }

        // Get file extension
        if let Some(extension) = path.extension() {
            let ext_str = extension
                .to_str()
                .context("Failed to convert extension to string")?
                .to_lowercase();

            // Determine category based on extension
            let category = match ext_str.as_str() {
                "jpg" | "png" => "Images".to_string(),
                "mp4" | "mkv" => "Videos".to_string(),
                "mp3" | "wav" => "Audio".to_string(),
                "pdf" | "docx" | "xlsx" | "pptx" | "csv" | "doc" => "Documents".to_string(),
                "zip" | "rar" | "7z" => "Compressed".to_string(),
                "exe" | "msi" => "Programs".to_string(),
                "" => "No Extension".to_string(),
                _ => ext_str,
            };

            let category_dir = dir.join(&category);

            // Skip if file is already in the correct directory
            if path.parent() == Some(category_dir.as_path()) {
                println!(
                    "Skipped (already organized): {}",
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                );
                continue;
            }

            // Create category directory if it doesn't exist
            fs::create_dir_all(&category_dir)
                .context("Failed to create category directory")?;

            // Move file to category directory
            if let Some(file_name) = path.file_name() {
                let dest_path = category_dir.join(file_name);
                fs::rename(&path, &dest_path)
                    .context("Failed to move file")?;
                println!(
                    "Moved: {} -> {}",
                    file_name.to_string_lossy(),
                    category
                );
            }

            organized_count += 1;
        }
    }

    println!("\nProcessed {} file(s)", organized_count);
    Ok(())
}
