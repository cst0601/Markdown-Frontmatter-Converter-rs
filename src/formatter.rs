use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Taxonomies {
    categories: Vec<String>,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(from="FrontmatterYaml")]
pub struct Frontmatter {
    title: String,
    author: Vec<String>,
    date: String,
    // Zola had tags and categories under taxonomies section in toml format
    taxonomies: Taxonomies,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct FrontmatterYaml {
    title: String,
    author: String,
    date: String,
    #[serde(flatten)]
    taxonomies: Taxonomies,
}

impl From<FrontmatterYaml> for Frontmatter {
    fn from(other: FrontmatterYaml) -> Frontmatter {
        Self {
            title: other.title,
            author: vec![other.author],
            date: other.date,
            taxonomies: other.taxonomies,
        }
    }
}

pub fn read_markdown_file(file_path: impl AsRef<Path>) -> String {
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

pub fn get_yaml_front_matter(content: &str) -> Option<(&str, &str)> {
    let content = content
        .trim_start()
        .strip_prefix("---")?;
    let content = content
        .strip_prefix("\n")
        .or_else(|| content.strip_prefix("\r\n"))?;
    let (yaml_content, markdown_content) = content.split_once("\n---")?;
    let yaml_content = yaml_content
        .strip_suffix("\r")
        .unwrap_or(yaml_content);
    Some((yaml_content, markdown_content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    use tempfile::NamedTempFile;

    const SOURCE_CONTEXT: &str = r"---
title: Leetcode 118 - Pascal's Triangle
author: chikuma
date: 1444-11-11 12:00:00 -0700
categories: [Leetcode]
tags: [Programming, Leetcode]
render_with_liquid: false
---

[Link to something](https://guthib.com)

# Title Goes Here

* Some content
* Some more content!

## Some more title

```cpp
auto ptr = std::make_unique<AwesomeStuff>();
```
";

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
        let content = r"---
title: Leetcode 118 - Pascal's Triangle
author: chikuma
---
# Markdown starts here
* point 1
* point 2
";
        let (yaml, markdown) = get_yaml_front_matter(content).unwrap();

        assert_eq!(yaml, r"title: Leetcode 118 - Pascal's Triangle
author: chikuma");
        assert_eq!(markdown, r"
# Markdown starts here
* point 1
* point 2
");
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

    #[test]
    fn test_empty_file_returns_none() {
        assert_eq!(None, get_yaml_front_matter(""));
    }

    #[test]
    fn test_format_yaml_to_toml() {
        let (frontmatter, _markdown) = get_yaml_front_matter(SOURCE_CONTEXT).unwrap();
        let values = serde_yaml_ng::from_str::<Frontmatter>(frontmatter).unwrap();

        let toml_result = toml::to_string(&values);
        assert!(toml_result.is_ok(), "Failed to serialize frontmatter to TOML");
    }
}
