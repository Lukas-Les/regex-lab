use regex::Regex;

pub fn search(re_str: &str, text: &str) -> Option<Vec<String>> {
    println!("regex:");
    println!("{re_str}");
    println!("current text:");
    println!("{text}");
    // Compile the regular expression
    let re = Regex::new(re_str).ok()?;

    let mut results: Vec<String> = Vec::new();

    for mat in re.find_iter(text) {
        let matched_str = mat.as_str().to_string();
        println!("Matched: {}", matched_str); // Debug output
        results.push(matched_str);
    }

    if results.is_empty() {
        return None;
    }

    println!("results:");
    for r in &results {
        println!("{}", r);
    }

    Some(results)
}


pub fn format_output(input: Option<Vec<String>>) -> String {
    match input {
        Some(v) => {
            v.join("\n")
        },
        None => "".to_string(),
    }
}

mod tests {
    use super::{search, format_output};

    #[test]
    fn test_search() {
        let text = "Some 420 None";
        let result = search(r"\w+", text);
        assert_eq!(result, Some(vec!["Some".to_string(), "420".to_string(), "None".to_string()]));
    }

    #[test]
    fn test_format_output() {
        let output = Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let result = format_output(output);
        assert_eq!(result, "a\nb\nc".to_string());
    }
}