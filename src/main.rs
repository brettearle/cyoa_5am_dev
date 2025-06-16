fn main() {
    println!("Hello, world!");
}

fn extract_value(json: &str, key: &str) -> Option<String> {
    let pattern = format!(r#""{}": "#, key);
    let start = json.find(&pattern)? + pattern.len();
    let remainder = json[start..].trim_start();

    if remainder.starts_with('"') {
        let value_start = start + 1;
        let value_end = json[value_start..].find('"')? + value_start;
        Some(json[value_start..value_end].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_value_happy_path() {
        let json = r#"
        {
            "text": "Follow a distant howl",
            "next": "2a"
        }"#;

        let text = extract_value(json, "text");
        let next = extract_value(json, "next");
        let expected_text = "Follow a distant howl";
        let expected_next = "2a";
        assert_eq!(text.unwrap(), expected_text);
        assert_eq!(next.unwrap(), expected_next)
    }
}
