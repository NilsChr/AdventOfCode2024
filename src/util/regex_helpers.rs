use regex::Regex;

pub fn extract_all_matches(pattern: &str, text: &str) -> Vec<String> {
    let re = Regex::new(pattern).expect("Invalid regex pattern");
    // Use find_iter to find all matches
    re.find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect()
}
