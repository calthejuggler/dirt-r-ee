use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use clipboard::{ClipboardContext, ClipboardProvider};
use colored::{Color, Colorize};
use rayon::prelude::*;

#[derive(Debug)]
pub struct FileTree {
    pub path: PathBuf,
    children: Vec<FileTree>,
    depth: usize,
}

impl FileTree {
    pub fn new(
        path: &Path,
        depth: usize,
        ignore: Arc<HashSet<String>>,
        hidden: bool,
        git_ignore: bool,
    ) -> FileTree {
        let children = if depth == 0 {
            vec![]
        } else {
            Self::find_files_and_directories(path, depth, ignore, hidden, git_ignore)
        };

        FileTree {
            path: path.to_path_buf(),
            children,
            depth,
        }
    }

    fn find_files_and_directories(
        path: &Path,
        depth: usize,
        ignore: Arc<HashSet<String>>,
        hidden: bool,
        git_ignore: bool,
    ) -> Vec<FileTree> {
        let mut children = Vec::new();

        if let Ok(entries) = std::fs::read_dir(path) {
            children = entries
                .filter_map(Result::ok)
                .par_bridge()
                .filter_map(|entry| {
                    let entry_path = entry.path();
                    let entry_str = entry_path.to_string_lossy();

                    let is_hidden = entry_path
                        .file_name()
                        .and_then(OsStr::to_str)
                        .map(|name| name.starts_with('.'))
                        .unwrap_or(false)
                        && !hidden;

                    let is_git_ignored =
                        !git_ignore && ignore.iter().any(|pattern| entry_str.contains(pattern));

                    if is_hidden || is_git_ignored {
                        None
                    } else {
                        Some(Self::new(
                            &entry_path,
                            depth + 1,
                            Arc::clone(&ignore),
                            hidden,
                            git_ignore,
                        ))
                    }
                })
                .collect();
        }

        children.sort_by(|a, b| {
            let a_is_dir = a.path.is_dir();
            let b_is_dir = b.path.is_dir();
            (b_is_dir.cmp(&a_is_dir)).then_with(|| a.path.cmp(&b.path))
        });

        children
    }

    pub fn print(&self, spacer: &str, prefix: &str) {
        let indent = spacer.repeat(self.depth);
        let name = self.path.file_name().unwrap().to_string_lossy();
        let suffix = if self.path.is_dir() { "/" } else { "" };

        let color: Color = match self.path.is_dir() {
            true => Color::BrightMagenta,
            false => Color::BrightCyan,
        };

        println!(
            "{}{}{}{}",
            indent,
            prefix,
            name.color(color),
            suffix.color(color)
        );

        for child in &self.children {
            child.print(spacer, prefix);
        }
    }

    pub fn copy(&self, spacer: &str, prefix: &str) {
        let mut output = String::new();
        self.build_string(&mut output, spacer, prefix);
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(output).unwrap();
    }

    pub fn write_to_file(&self, out_file: String, spacer: &str, prefix: &str) {
        let mut output = String::new();
        self.build_string(&mut output, spacer, prefix);
        std::fs::write(out_file, output).unwrap();
    }

    fn build_string(&self, output: &mut String, spacer: &str, prefix: &str) {
        let indent = spacer.repeat(self.depth);
        let name = self.path.file_name().unwrap().to_string_lossy();
        let suffix = if self.path.is_dir() { "/" } else { "" };

        let line = format!("{}{}{}{}", indent, prefix, name, suffix);
        output.push_str(&line);
        output.push('\n');

        for child in &self.children {
            child.build_string(output, spacer, prefix);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

    fn create_test_environment() -> (PathBuf, HashSet<String>) {
        let test_dir = tempdir().unwrap();
        let test_path = test_dir.path().to_path_buf();

        let subdir = test_path.join("subdir");
        fs::create_dir(&subdir).unwrap();

        let hidden_dir = test_path.join(".hidden_dir");
        fs::create_dir(hidden_dir).unwrap();

        let file = subdir.join("file.txt");
        let _file_handle = File::create(file).unwrap();

        let hidden_file = subdir.join(".hidden_file.txt");
        let _hidden_file_handle = File::create(hidden_file).unwrap();

        let gitignore_contents = "ignored_file.txt";
        let gitignore_file = test_path.join(".gitignore");
        let mut file = File::create(gitignore_file).unwrap();
        writeln!(file, "{}", gitignore_contents).unwrap();

        let mut ignore_set = HashSet::new();
        ignore_set.insert(gitignore_contents.to_string());

        (test_path, ignore_set)
    }

    #[test]
    fn test_new_ignoring_hidden_files() {
        let (test_path, ignore_set) = create_test_environment();
        let file_tree = FileTree::new(&test_path, 2, Arc::new(ignore_set), true, false);

        assert!(!file_tree
            .children
            .iter()
            .any(|child| child.path.to_string_lossy().contains(".hidden")));
    }

    #[test]
    fn test_new_with_git_ignore() {
        let (test_path, ignore_set) = create_test_environment();
        let file_tree = FileTree::new(&test_path, 2, Arc::new(ignore_set), false, true);

        assert!(!file_tree
            .children
            .iter()
            .any(|child| child.path.ends_with("ignored_file.txt")));
    }

    #[test]
    fn test_new_with_depth_limit() {
        let (test_path, ignore_set) = create_test_environment();
        let file_tree = FileTree::new(&test_path, 0, Arc::new(ignore_set), false, false);

        assert!(file_tree.children.is_empty());
    }
}
