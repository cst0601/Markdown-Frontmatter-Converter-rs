use std::fs;
use std::io::ErrorKind;
use std::path::Path;


fn read_markdown_file(file_path: impl AsRef<Path>) -> String {
    match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            eprintln!();
            String::new()
        }
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
            String::new()
        }
    }
}

fn get_yaml_front_matter(content: &str) -> Option<&str> {
    let content = content
        .trim_start()
        .strip_prefix("---")?;
    let content = content
        .strip_prefix("\n")
        .or_else(|| content.strip_prefix("\r\n"))?;
    let (yaml_content, _) = content.split_once("\n---")?;
    let yaml_content = yaml_content
        .strip_suffix("\r")
        .unwrap_or(yaml_content);
    Some(yaml_content)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_file_success() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file.");
        let markdown_content = "
            # Markdown title
            * I really hope the tests would pass.
        ";
        write!(temp_file, "{}", markdown_content).expect("Failed to write to temp file.");

        let result = read_markdown_file(temp_file.path());
        assert_eq!(result, markdown_content);
    }

    #[test]
    fn test_read_non_existent_file() {
        let missing_path = Path::new("this_file_should_not_exist.md");
        let result = read_markdown_file(missing_path);
        assert_eq!(result, "");
    }

    #[test]
    fn test_get_yaml_front_matter() {
        let content = read_markdown_file("data/test.md");
        let result = get_yaml_front_matter(&content).unwrap();

        assert_eq!(result, r"title: Leetcode 118 - Pascal's Triangle
author: chikuma");
    }

    #[test]
    fn test_no_yaml_in_front_matter_returns_none() {
        let markdown_content = "
        # Some title
        This markdown does not have any yaml front matter.
        ";

        assert_eq!(None, get_yaml_front_matter(markdown_content));
    }

    #[test]
    fn test_only_yaml_starting_delimeter_returns_none() {
        let markdown_content = "---
        # Some title
        This markdown only contains yaml starting delimeter
        ";

        assert_eq!(None, get_yaml_front_matter(markdown_content));
    }
}