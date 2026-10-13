use std::fs;

pub fn find_markdown_files(dir: &str)
    -> Result<Vec<std::path::PathBuf>, std::io::Error>
{
    let markdown_files = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .map_or(false, |ext| ext == "md")
        })
        .collect();

    Ok(markdown_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::{TempDir, tempdir};

    const FILE_EXTENSIONS: &[&'static str] = &["md", "tar.gz", "rs", "xmd"];

    // temp dir destructs after it goes out of scope, therefore we need to pass
    // ownership for this to not happen
    fn create_temp_files() -> TempDir {
        let dir = tempdir().unwrap();

        for file_extension in FILE_EXTENSIONS {
            for i in 0..5 {
                let temp_file_path = dir.path().join(format!("temp_file_{}.{}", i, file_extension));
                let _ = File::create(temp_file_path);
            }
        }

        let files = find_markdown_files(&dir.path().display().to_string()).unwrap();
        assert_eq!(files.len(), 5);
        dir
    }

    #[test]
    fn test_find_markdown_files() {
        let path = create_temp_files();
        let path_string = path.path().display().to_string();
        let markdown_files = find_markdown_files(&path_string).unwrap();

        assert_eq!(markdown_files.len(), 5);
    }
}
