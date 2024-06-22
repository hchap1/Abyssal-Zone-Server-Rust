pub fn split_with_delimiter(input: String, delimiter: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;

    while let Some(pos) = input[start..].find(delimiter) {
        let end = start + pos + delimiter.len();
        let component: String = input[start..end].to_string();
        if !component.is_empty() && !component.chars().all(|c| c == '\0') {
            result.push(component);
        }
        start = end;
    }

    if start < input.len() {
        let component: String = input[start..].to_string();
        if !component.is_empty() && !component.chars().all(|c| c == '\0') {
            result.push(component);
        }
    }

    result
}