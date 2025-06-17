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

fn extract_object(json: &str) -> String {
    let s = json.trim_start();
    let mut brace_count = 0;
    let chars = s.chars();
    let mut result = String::new();
    for c in chars {
        if c == '{' {
            brace_count += 1;
            result.push(c);
            continue;
        } else if c == '}' {
            brace_count -= 1;
            result.push(c);
            if brace_count == 0 {
                break;
            }
        }
        result.push(c)
    }
    result
}

fn extract_keys(json: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_key = false;
    let chars = json.chars();
    for c in chars {
        if c == '{' {
            depth += 1;
            continue;
        } else if c == '}' {
            depth -= 1;
            continue;
        }
        if !in_key && c == '"' && depth == 1 {
            in_key = !in_key;
            continue;
        } else if in_key && c != '"' {
            current.push(c);
            continue;
        } else if in_key && c == '"' {
            in_key = !in_key;
            keys.push(current.clone());
            current.clear();
            continue;
        }
    }

    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keys_happy_path() {
        let json = r#"
        {
            "key_one": "not under test",
            "key_two": { "not_under_test": "no test"},
            "key_three": "no comma seperation"
        }
        "#;
        let expected = vec![
            "key_one".to_string(),
            "key_two".to_string(),
            "key_three".to_string(),
        ];
        let got = extract_keys(json);
        assert_eq!(got, expected)
    }

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

    #[test]
    fn test_extract_object_happy_path() {
        let json = r#"
            {
                "end_1": "You survive the night, but the forest keeps its secrets. You are never the same again.",
                "end_2": "You wander endlessly. Eventually, the forest swallows you whole.",
                "end_3": "You escape the forest and find refuge. Your story becomes legend."
            }
            }
        "#;

        let got = extract_object(json);
        let expected = r#"{
                "end_1": "You survive the night, but the forest keeps its secrets. You are never the same again.",
                "end_2": "You wander endlessly. Eventually, the forest swallows you whole.",
                "end_3": "You escape the forest and find refuge. Your story becomes legend."
            }"#;

        assert_eq!(got, expected)
    }
}
