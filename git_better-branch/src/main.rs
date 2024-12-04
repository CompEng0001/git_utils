//! A Rust program that scans directories for Git repositories, checks branches, 
//! and displays their ahead/behind status relative to the main branch.
//!
//! This program uses the `walkdir` crate for directory traversal, the `regex` crate 
//! for parsing branch data, and executes Git commands to gather repository information.

use std::env;
use std::process::{Command, Stdio};
use std::str;
use walkdir::WalkDir;
use regex::Regex;

/// ANSI escape codes for colored terminal output
const RED: &str = "\x1b[0;31m";
const GREEN: &str = "\x1b[0;32m";
const NO_COLOR: &str = "\x1b[0m";
const BLUE: &str = "\x1b[0;34m";
const YELLOW: &str = "\x1b[0;33m";

/// Runs a Git command with the provided arguments and returns its trimmed output.
///
/// # Arguments
///
/// * `args` - A slice of string slices containing the Git command arguments.
///
/// # Returns
///
/// A `String` containing the trimmed output of the Git command.
fn run_command_and_trim(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("Failed to execute git command");

    str::from_utf8(&output.stdout)
        .expect("Command output is not valid UTF-8")
        .trim()
        .to_string()
}

/// Counts the commits that are ahead and behind between two branches.
///
/// # Arguments
///
/// * `dir` - The path to the Git repository.
/// * `branch` - The branch being compared.
/// * `base_branch` - The base branch for comparison.
///
/// # Returns
///
/// A tuple `(ahead, behind)` where `ahead` is the number of commits the branch is ahead
/// of the base branch, and `behind` is the number of commits it is behind.
fn count_commits(dir: &str, branch: &str, base_branch: &str) -> (i32, i32) {
    let output = run_command_and_trim(&[
        "-C",
        dir,
        "rev-list",
        "--left-right",
        "--count",
        &format!("{}...{}", base_branch, branch),
    ]);

    let parts: Vec<&str> = output.split('\t').collect();
    let behind = parts[0].parse().unwrap_or(0);
    let ahead = parts[1].parse().unwrap_or(0);
    (ahead, behind)
}

/// Checks if a given directory is a Git repository.
///
/// # Arguments
///
/// * `path` - The path to the directory being checked.
///
/// # Returns
///
/// `true` if the directory is a Git repository, `false` otherwise.
fn is_git_repo(path: &str) -> bool {
    let status = Command::new("git")
        .args(&["-C", path, "rev-parse", "--is-inside-work-tree"])
        .stderr(Stdio::null())
        .output();

    match status {
        Ok(output) => output.status.success(),
        Err(_) => {
            eprintln!("Failed to check if '{}' is a git repository", path);
            false
        }
    }
}

/// Processes a Git repository and displays information about its branches.
///
/// # Arguments
///
/// * `dir` - The path to the Git repository.
fn process_repo(dir: &str) {
    // Get the repository name by retrieving the remote URL.
    let repo_name = run_command_and_trim(&["-C", dir, "remote", "get-url", "origin"])
        .replace(".git", "");

    println!("Repo: {}", repo_name);

    // Get the main branch (HEAD).
    let main_branch = run_command_and_trim(&["-C", dir, "rev-parse", "HEAD"]);

    // Print the header for branch information.
    println!(
        "{}{:5} {}{:6} {}{:30} {}{:20} {}{:40}",
        GREEN, "Ahead", RED, "Behind", BLUE, "Branch", YELLOW, "Last Commit", NO_COLOR, " "
    );
    println!(
        "{}{:5} {}{:6} {}{:30} {}{:20} {}{:40}",
        GREEN, "-----", RED, "------", BLUE, "------------------------------", YELLOW, "-------------------", NO_COLOR, " "
    );

    // Retrieve branch information using `git for-each-ref`.
    let branches_output = run_command_and_trim(&[
        "-C",
        dir,
        "for-each-ref",
        "--sort=-authordate",
        "--format=%(objectname:short)@%(refname:short)@%(committerdate:relative)",
        "refs/heads/",
    ]);

    // Regex to parse branch data.
    let branch_regex = Regex::new(r"([^\@]+)@([^\@]+)@([^\@]+)").unwrap();
    let branches = branches_output.trim().lines();

    for branchdata in branches {
        if let Some(caps) = branch_regex.captures(branchdata) {
            let sha = &caps[1];
            let branch = &caps[2];
            let time = &caps[3];

            if branch != main_branch {
                let (ahead, behind) = count_commits(dir, sha, &main_branch);
                println!(
                    "{}{:5} {}{:6} {}{:30} {}{:20} {}{:40}",
                    GREEN, ahead, RED, behind, BLUE, branch, YELLOW, time, NO_COLOR, ""
                );
            }
        }
    }
    println!();
}

/// The entry point of the program. Scans directories for Git repositories and processes them.
fn main() {
    // Parse command-line arguments for depth.
    let args: Vec<String> = env::args().collect();
    let depth = args.get(1).and_then(|d| d.parse().ok()).unwrap_or(0);

    // Get the current working directory.
    let current_dir = env::current_dir().expect("Failed to get current directory");

    println!("Scanning '{}' up to a depth of {}", current_dir.display(), depth);

    let mut found_repos = false;

    // Walk directories using `walkdir`.
    for entry in WalkDir::new(&current_dir)
        .min_depth(0)
        .max_depth(depth)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_dir() && is_git_repo(path.to_str().unwrap()) {
            found_repos = true;
            process_repo(path.to_str().unwrap());
        }
    }

    // If no repositories are found, notify the user.
    if !found_repos {
        println!("No git repositories found in '{}'.", current_dir.display());
    }
}
