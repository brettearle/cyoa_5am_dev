fn main() {
    println!("Hello, world!");
}

fn extract_value(json: &str, key: &str) -> String {
    let pattern = format!(r#""{}": "#, key);
    let start = json.find(&pattern).unwrap() + pattern.len();
    let value_start = start + 1;
    let value_end = json[value_start..].find('"').unwrap() + value_start;
    json[value_start..value_end].to_string()
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
        assert_eq!(text, expected_text);
        assert_eq!(next, expected_next)
    }
}
