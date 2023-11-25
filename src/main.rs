mod cli;
mod file_tree;
mod utils;

use std::env;
use std::path::Path;
use std::sync::Arc;

use clap::Parser;

use crate::cli::Args;
use crate::file_tree::FileTree;
use crate::utils::read_gitignore;

fn main() {
    let args = Args::parse();

    let dir_str = args
        .dir
        .unwrap_or_else(|| env::current_dir().unwrap().to_string_lossy().to_string());
    let path = Path::new(&dir_str);

    let spacer = args.spacer.unwrap_or_else(|| "    ".to_string());
    let prefix = args.prefix.unwrap_or_else(|| "- ".to_string());

    if !path.exists() {
        eprintln!("No such file or directory");
        return;
    }

    let ignore = Arc::new(read_gitignore(path));
    let file_tree = FileTree::new(path, 1, ignore, args.include_hidden, args.git_ignored);

    if args.copy {
        file_tree.copy(&spacer, &prefix);
    } else {
        file_tree.print(&spacer, &prefix);
    }

    if args.out_file.is_some() {
        file_tree.write_to_file(args.out_file.unwrap(), &spacer, &prefix);
    }
}
