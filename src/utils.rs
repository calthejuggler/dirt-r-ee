use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn read_gitignore(path: &Path) -> HashSet<String> {
    fs::read_to_string(path.join(".gitignore"))
        .map(|content| {
            content
                .lines()
                .filter(|line| !line.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_gitignore_with_valid_file() {
        let dir = tempdir().unwrap();
        let gitignore_path = dir.path().join(".gitignore");
        let mut file = File::create(&gitignore_path).unwrap();
        writeln!(file, "target/\n*.log\n").unwrap();

        let gitignore = read_gitignore(dir.path());
        assert!(gitignore.contains("target/"));
        assert!(gitignore.contains("*.log"));
    }

    #[test]
    fn test_read_gitignore_with_empty_file() {
        let dir = tempdir().unwrap();
        let gitignore_path = dir.path().join(".gitignore");
        File::create(&gitignore_path).unwrap();

        let gitignore = read_gitignore(dir.path());
        assert!(gitignore.is_empty());
    }

    #[test]
    fn test_read_gitignore_with_non_existent_file() {
        let dir = tempdir().unwrap();

        let gitignore = read_gitignore(dir.path());
        assert!(gitignore.is_empty());
    }

    #[test]
    fn test_read_gitignore_with_file_having_empty_lines() {
        let dir = tempdir().unwrap();
        let gitignore_path = dir.path().join(".gitignore");
        let mut file = File::create(&gitignore_path).unwrap();
        writeln!(file, "\n\nignored_file\n\n").unwrap();

        let gitignore = read_gitignore(dir.path());
        assert_eq!(gitignore.len(), 1);
        assert!(gitignore.contains("ignored_file"));
    }
}
