use validator::ValidationErrors;

pub fn format_validation_errors(errors: &ValidationErrors) -> String {
  let map_error = errors
    .field_errors()
    .iter()
    .map(|(field, errors)| {
      let formatted: Vec<String> = errors
        .iter()
        .map(|e| {
          let mut parts: Vec<String> = Vec::new();
          parts.push(e.code.to_string());

          if let Some(msg) = &e.message {
            parts.push(msg.to_string());
          }

          if let Some(v) = e.params.get("value") {
            parts.push(format!("value={v}"));
          }

          for (k, v) in e.params.iter().filter(|(k, _)| **k != "value") {
            parts.push(format!("{k}={v}"));
          }

          parts.join("|")
        })
        .collect();

      if formatted.is_empty() {
        format!("{field}|Invalid value")
      } else {
        format!("{field}|{}", formatted.join(", "))
      }
    })
    .collect::<Vec<String>>()
    .join(";");

  if map_error.is_empty() {
    errors.to_string()
  } else {
    map_error
  }
}
