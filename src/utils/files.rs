use axum::body::Bytes;
use axum_typed_multipart::FieldData;
use std::path::{Path, PathBuf};
use tokio::fs;

const PREFIX_PATH: &str = "public/uploads";

pub fn get_path<P: AsRef<Path>>(file_path: P) -> PathBuf {
    Path::new(PREFIX_PATH).join(file_path)
}

// get file name from path
pub fn get_file_name_from_path<P: AsRef<Path>>(file_path: P) -> Option<String> {
    let full_path = Path::new(PREFIX_PATH).join(file_path);

    full_path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name_str| name_str.to_string())
}

// Save file to disk
pub async fn save_file<P: AsRef<Path>>(
    file_name: P, // filename or path e.g. /tmp/file.txt
    file: FieldData<Bytes>,
    overwrite: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    // Convert file_name to Path for easier manipulation
    let path = get_path(file_name.as_ref());

    // Check if file exists and overwrite is false
    if overwrite && path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "FILE_EXISTS",
        )));
    }

    // Ensure that the parent directory exists, create it if it does not
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).await?;
        }
    }
    // Persist file contents
    fs::write(path.clone(), &file.contents).await?;
    Ok(path.to_string_lossy().to_string())
}

// read file from disk
pub async fn read_file<P: AsRef<Path>>(file_name: P) -> Result<Bytes, Box<dyn std::error::Error>> {
    let path = get_path(file_name.as_ref());
    let data = fs::read(path).await?;
    Ok(Bytes::from(data))
}

// delete file from disk and clean up empty parent folders up to PREFIX_PATH
pub async fn delete_file<P: AsRef<Path>>(file_path: P) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::ErrorKind;

    let path = get_path(file_path.as_ref());

    // Remove the file; ignore if it doesn't exist
    if let Err(e) = fs::remove_file(&path).await {
        if e.kind() != ErrorKind::NotFound {
            return Err(Box::new(e));
        }
    }

    // Walk up the directory tree and remove empty folders until PREFIX_PATH or non-empty dir
    let mut current = path.parent();
    let root = Path::new(PREFIX_PATH).canonicalize()?;

    while let Some(dir) = current {
        // Stop at the storage root
        if dir == root {
            break;
        }

        match fs::remove_dir(dir).await {
            Ok(_) => {
                current = dir.parent();
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // Directory already gone; proceed upward
                current = dir.parent();
            }
            Err(e)
                if e.kind() == ErrorKind::DirectoryNotEmpty
                    || e.kind() == ErrorKind::PermissionDenied =>
            {
                break; // Not empty or cannot delete; stop cleaning up
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(())
}
