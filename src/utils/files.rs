use anyhow::{Result, bail};
use axum::body::Bytes;
use std::path::{Path, PathBuf};
use tokio::fs;

const PREFIX_PATH: &str = "public/uploads";

pub fn get_path<P: AsRef<Path>>(file_path: P) -> PathBuf {
  Path::new(PREFIX_PATH).join(file_path)
}

pub fn get_file_name_from_path<P: AsRef<Path>>(file_path: P) -> Option<String> {
  let full_path = Path::new(PREFIX_PATH).join(file_path);

  full_path
    .file_name()
    .and_then(|name| name.to_str())
    .map(|name_str| name_str.to_string())
}

pub async fn save_file_from_bytes<P: AsRef<Path>>(
  file_name: P,
  contents: &Bytes,
  overwrite: bool,
) -> Result<String> {
  let path = get_path(file_name.as_ref());

  if !overwrite && path.exists() {
    bail!("FILE_EXISTS");
  }

  if let Some(parent) = path.parent()
    && !parent.exists()
  {
    fs::create_dir_all(parent).await?;
  }

  fs::write(&path, contents).await?;
  Ok(path.to_string_lossy().to_string())
}

pub async fn read_file<P: AsRef<Path>>(file_name: P) -> Result<Bytes> {
  let path = get_path(file_name.as_ref());
  let data = fs::read(path).await?;
  Ok(Bytes::from(data))
}

pub async fn delete_file<P: AsRef<Path>>(file_path: P) -> Result<()> {
  use std::io::ErrorKind;

  let path = get_path(file_path.as_ref());

  if let Err(e) = fs::remove_file(&path).await
    && e.kind() != ErrorKind::NotFound
  {
    return Err(e.into());
  }

  let mut current = path.parent();
  let root = Path::new(PREFIX_PATH).canonicalize()?;

  while let Some(dir) = current {
    if dir == root {
      break;
    }

    match fs::remove_dir(dir).await {
      Ok(_) => {
        current = dir.parent();
      }
      Err(e) if e.kind() == ErrorKind::NotFound => {
        current = dir.parent();
      }
      Err(e)
        if e.kind() == ErrorKind::DirectoryNotEmpty
          || e.kind() == ErrorKind::PermissionDenied =>
      {
        break;
      }
      Err(e) => return Err(e.into()),
    }
  }

  Ok(())
}
