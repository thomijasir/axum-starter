use std::collections::HashMap;

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
