use std::collections::HashMap;
use std::path::Path;

pub fn query_to_string(params: HashMap<String, String>) -> String {
  let query_string = params
    .iter()
    .map(|(key, value)| format!("{}={}", key, value))
    .collect::<Vec<_>>()
    .join("&");
  format!("?{}", query_string)
}

pub fn slugify(input: &str) -> String {
  input
    .chars()
    .fold(
      (String::new(), false),
      |(mut output, prev_was_hyphen), c| {
        if c.is_alphanumeric() {
          output.push(c.to_ascii_lowercase());
          (output, false)
        } else if !prev_was_hyphen && !output.is_empty() {
          output.push('-');
          (output, true)
        } else {
          (output, prev_was_hyphen)
        }
      },
    )
    .0
    .trim_matches('-')
    .to_string()
}

pub fn slugify_filename(filename: &str) -> String {
  let path = Path::new(filename);
  let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
  let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

  let slug_stem = slugify(stem);
  let slug_stem = if slug_stem.is_empty() {
    "file".to_string()
  } else {
    slug_stem
  };

  if ext.is_empty() {
    slug_stem
  } else {
    format!("{}.{}", slug_stem, ext.to_ascii_lowercase())
  }
}

#[cfg(test)]
mod tests {
  use super::slugify_filename;

  #[test]
  fn test_basic_slugify_filename() {
    assert_eq!(slugify_filename("My Document.PDF"), "my-document.pdf");
  }

  #[test]
  fn test_spaces_and_special_chars() {
    assert_eq!(slugify_filename("hello world!.txt"), "hello-world.txt");
  }

  #[test]
  fn test_no_extension() {
    assert_eq!(slugify_filename("README"), "readme");
  }

  #[test]
  fn test_empty_stem_falls_back_to_file() {
    assert_eq!(slugify_filename("!!.png"), "file.png");
  }

  #[test]
  fn test_preserves_lowercase_extension() {
    assert_eq!(slugify_filename("photo.JPEG"), "photo.jpeg");
  }

  #[test]
  fn test_unicode_stem() {
    // slugify keeps alphanumeric unicode chars, lowercased
    assert_eq!(slugify_filename("café résumé.pdf"), "café-résumé.pdf");
  }
}
