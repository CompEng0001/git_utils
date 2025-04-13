//! Git repository inspector.
//!
//! This module provides functionality to extract the GitHub repository
//! owner and name from the local `.git/config` file.
//!
//! If the program is not running inside a Git repository, it supports falling
//! back to parsing command-line arguments for owner and repo name.
use regex::Regex;
use std::{env, fs, process::exit};
use std::process::Command;

/// Attempts to determine the repository owner and name.
///
/// This function will first check if the current working directory is inside a Git
/// repository using `git rev-parse --is-inside-work-tree`. If so, it reads the
/// `.git/config` file and parses the `url` entry for the `origin` remote to extract
/// the GitHub owner and repository name.
///
/// If the directory is not inside a Git repo, this function falls back to expecting
/// two arguments: the repository owner at position 1 and the repository name at position 2.
///
/// # Arguments
///
/// * `args` - A vector of command-line arguments. Only used if not in a Git repo.
///
/// # Returns
///
/// A tuple `(owner, repo)` as `String` values.
///
/// # Panics / Exits
///
/// This function will terminate the program with an error message if:
/// - The Git command fails or indicates it's not inside a repo
/// - The `.git/config` file cannot be read or parsed
/// - Not enough CLI arguments are provided when required
pub async fn get_repo_path(args: Vec<String>) -> (String, String) {
    let current_dir = env::current_dir().expect("Unable to get current directory");
    let output = Command::new("git")
        .current_dir(&current_dir)
        .args(&["rev-parse", "--is-inside-work-tree"])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true" {
        let config_path = current_dir.join(".git").join("config");
        if !config_path.exists() {
            eprintln!(".git/config not found. Are you in a Git repository?");
            exit(1);
        }
        let config_content = fs::read_to_string(config_path).unwrap_or_else(|e| {
            eprintln!("Unable to read .git/config: {}", e);
            exit(1);
        });

        let owner_re = Regex::new(r#"url\s*=\s*git@github\.com:(\w+)/([\w-]+)\.git"#).unwrap();
        let caps = owner_re.captures(&config_content).unwrap_or_else(|| {
            eprintln!("Failed to extract repository owner and name from .git/config");
            exit(1);
        });

        let owner = caps.get(1).unwrap().as_str().to_string();
        let name = caps.get(2).unwrap().as_str().to_string();
        (owner, name)
    } else if args.len() == 3 {
        (args[1].clone(), args[2].clone())
    } else {
        eprintln!("Position 1: Owner, position 2: repo name, or be in a git repo");
        exit(1);
    }
}