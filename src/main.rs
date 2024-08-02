use std::env;
use std::fs::{self, File};
use std::io::{Write, Error, BufWriter};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <repository_path> [output_file]", args[0]);
        std::process::exit(1);
    }

    let repo_path = &args[1];
    let output_path = args.get(2).map(String::as_str).unwrap_or("flattened_repo.txt");

    flatten_repository(repo_path, output_path)?;

    println!("Repository flattened successfully. Output: {}", output_path);
    Ok(())
}

fn flatten_repository(repo_path: &str, output_path: &str) -> Result<(), Error> {
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let repo_path = PathBuf::from(repo_path);
    let output_path = PathBuf::from(output_path);

    visit_dirs(&repo_path, &mut writer, &output_path)?;
    writer.flush()?;
    Ok(())
}

fn visit_dirs(dir: &Path, writer: &mut BufWriter<File>, output_path: &Path) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path == *output_path {
                continue;
            }
            if should_skip_file_or_directory(&path) {
                continue;
            }
            if path.is_dir() {
                visit_dirs(&path, writer, output_path)?;
            } else {
                write_file_content(&path, writer)?;
            }
        }
    }
    Ok(())
}

fn write_file_content(file_path: &Path, writer: &mut BufWriter<File>) -> Result<(), Error> {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let content = fs::read_to_string(file_path)?;

    writeln!(writer, "{}", file_name)?;
    writeln!(writer, "{}", content)?;
    writeln!(writer, "--------")?;

    Ok(())
}

fn should_skip_file_or_directory(path: &Path) -> bool {
    if path.is_dir() {
        should_skip_directory(path)
    } else {
        should_skip_file(path)
    }
}

fn should_skip_directory(path: &Path) -> bool {
    let skip_dirs = [".git", "node_modules", "target"];
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| skip_dirs.contains(&name))
        .unwrap_or(false)
}

fn should_skip_file(path: &Path) -> bool {
    let skip_extensions = [
        "exe", "dll", "so", "dylib",
        "jpg", "jpeg", "png", "gif", "bmp",
        "mp3", "wav", "ogg",
        "mp4", "avi", "mov",
        "zip", "tar", "gz", "7z",
    ];

    let skip_filenames = [
        "package-lock.json",
        "package-lock.yml",
        "yarn.lock",
        "Cargo.lock",
        "LICENSE"
    ];

    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if skip_filenames.contains(&file_name) {
            return true;
        }
    }
    
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| skip_extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}