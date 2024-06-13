//! This library provides utilities for the CLI tools.
//! 
//! Example:
//! ```
//! use leranin_rust_through_coursera::read_stdin;
//! let input = read_stdin()
//! ```
//! 
//! #Panics:
//! The `read_stdin()` function will panic if it failes to read a line.

use std::io::{BufRead, BufReader};

/// This is a CLI function that reads a line and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read!"
/// 
/// Example:
/// ```
/// let input = read_stdin()
/// ```

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();

    reader.read_line(&mut line).expect("Failed to read!");
    line.trim().to_string()
}