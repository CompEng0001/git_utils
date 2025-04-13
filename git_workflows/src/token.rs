//! GitHub token loader.
//!
//! This module is responsible for retrieving a GitHub personal access token (PAT)
//! from a file whose path is specified by the `GITHUB_TOKEN_PATH` environment variable.
//!
//! The token is required to authenticate API requests made to the GitHub REST API.
//!
//! # Environment Variable
//! - `GITHUB_TOKEN_PATH`: must point to a plaintext file containing the token (with no trailing newline if possible).
//!
//! # Errors
//! This module will terminate the process with an error message if:
//! - The environment variable is not set
//! - The file does not exist at the specified path
//! - The file cannot be read
use std::{env, fs, path::PathBuf, process::exit};

/// Reads the GitHub personal access token from a file specified by the `GITHUB_TOKEN_PATH` environment variable.
///
/// # Returns
/// A trimmed `String` containing the token.
///
/// # Panics
/// This function will terminate the program with an error message if:
/// - The `GITHUB_TOKEN_PATH` environment variable is not set
/// - The specified file does not exist
/// - The file cannot be read
pub fn get_github_token() -> String {
    let github_token_path = PathBuf::from(
        env::var("GITHUB_TOKEN_PATH").expect("GITHUB_TOKEN_PATH environment variable not set, path to GitHub token"),
    );

    if !github_token_path.exists() {
        eprintln!("GitHub token file does not exist at path: {:?}", github_token_path);
        exit(1);
    }

    fs::read_to_string(&github_token_path)
        .expect("Unable to read GitHub token")
        .trim()
        .to_string()
}