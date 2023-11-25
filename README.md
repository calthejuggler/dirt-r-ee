<p align="center">
  <img src="https://github.com/calthejuggler/dirt-r-ee/assets/30095183/7ec6360e-2a01-49b4-98f8-e5ffbe9ee086" width="400" />
</p>

# Dirt-r-ee

A CLI tool for printing the structure of a given directory in a tree.

## Overview

Dirt-r-ee is a command-line interface (CLI) tool built in Rust that generates a tree representation of a directory
structure. It's designed to provide a clear view of file and directory arrangements, supporting various features like
including hidden files, respecting `.gitignore` rules, and customizable spacing.

## Features

- **Tree Generation**: Visually represents the directory structure in a tree format.
- **Hidden Files**: Option to include hidden files in the tree.
- **Git Ignore Support**: Respects `.gitignore` files to exclude certain files and directories.
- **Custom Spacing**: Allows customizing the spacing in the tree structure for better readability.
- **Clipboard Support**: Provides an option to copy the generated tree to the clipboard.
- **Parallel Processing**: Leverages Rust's `rayon` crate for efficient directory traversal.

## Installation

To install Dirt-r-ee, you need Rust and Cargo installed on your system. Follow these steps:

1. Clone the repository: `git clone https://github.com/calthejuggler/dirt-r-ee.git`
2. Change to the directory: `cd dirt-r-ee`
3. Build the project: `cargo build --release`
4. The executable can be found in `target/release`

## Usage

Run Dirt-r-ee from the command line. The basic command structure is as follows:

```
dirt-r-ee [OPTIONS]
```

### Options

- `-d`, `--dir <DIRECTORY>`: Specify the directory to print. Defaults to the current directory.
- `-s`, `--spacer <SPACER>`: Custom spacer for indentation. Default is four spaces.
- `-i`, `--include-hidden`: Include hidden files and directories in the output.
- `-g`, `--git-ignored`: Include files and directories specified in `.gitignore`.
- `-c`, `--copy`: Copy the output to the clipboard instead of printing it.

### Examples

- Display the current directory tree: `dirt-r-ee`
- Display a specific directory: `dirt-r-ee -d /path/to/dir`
- Include hidden files: `dirt-r-ee -i`
- Copy the output to clipboard: `dirt-r-ee -c`

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

Distributed under the MIT License. See `LICENSE` for more information.
