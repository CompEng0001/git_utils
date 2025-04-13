//! Git-related utility functions used by the `git-better-branch` tool.
//!
//! This module provides low-level interaction with the Git CLI to detect repositories,
//! list branches, compare commit counts, and suggest branch names based on fuzzy matching.

use std::process::{Command, Stdio};
use std::str;
use strsim::levenshtein;

/// Executes a Git command and returns the trimmed UTF-8 output.
///
/// # Arguments
///
/// * `args` - A slice of command-line arguments to pass to `git`.
///
/// # Returns
///
/// A `String` containing the trimmed standard output from the Git command.
///
/// # Panics
///
/// Panics if the command fails to run or the output is not valid UTF-8.
pub fn run_command_and_trim(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("Failed to execute git command");

    str::from_utf8(&output.stdout)
        .expect("Command output is not valid UTF-8")
        .trim()
        .to_string()
}

/// Checks whether the given path is a Git repository.
///
/// # Arguments
///
/// * `path` - The filesystem path to check.
///
/// # Returns
///
/// `true` if the directory is a Git repository, `false` otherwise.
pub fn is_git_repo(path: &str) -> bool {
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

/// Attempts to determine the default base branch for a repository.
///
/// It prefers `main`, then `master`, based on existing local branches.
///
/// # Arguments
///
/// * `dir` - The Git repository directory.
///
/// # Returns
///
/// `Some(branch_name)` if a known default base branch is found, or `None` if neither exists.
pub fn get_default_base_branch(dir: &str) -> Option<String> {
    let branches = run_command_and_trim(&[
        "-C", dir,
        "for-each-ref",
        "refs/heads/",
        "--format=%(refname:short)",
    ]);

    for name in ["main", "master"] {
        if branches.lines().any(|b| b.trim() == name) {
            return Some(name.to_string());
        }
    }

    None
}

/// Compares two branches and returns the number of commits `ahead` and `behind`.
///
/// # Arguments
///
/// * `dir` - The Git repository directory.
/// * `branch` - The target branch to compare.
/// * `base_branch` - The reference branch to compare against.
///
/// # Returns
///
/// A tuple `(ahead, behind)` representing how many commits the target branch
/// is ahead of or behind the base branch.
///
/// # Notes
///
/// This uses `git rev-list --left-right --count base...branch`.
pub fn count_commits(dir: &str, branch: &str, base_branch: &str) -> (i32, i32) {
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

/// Lists all branch names (local and remote) in a repository.
///
/// # Arguments
///
/// * `dir` - The Git repository directory.
///
/// # Returns
///
/// A `Vec<String>` of all branch names (e.g., `main`, `origin/dev`).
pub fn list_all_branch_names(dir: &str) -> Vec<String> {
    let output = run_command_and_trim(&[
        "-C", dir,
        "for-each-ref",
        "--format=%(refname:short)",
    ]);
    output.lines().map(|l| l.trim().to_string()).collect()
}

/// Suggests similar branch names using Levenshtein distance for fuzzy matching.
///
/// # Arguments
///
/// * `input` - The branch name to match against.
/// * `candidates` - A list of existing branch names.
///
/// # Returns
///
/// A vector of suggested branch names with small edit distances (<= 3).
///
/// # Example
///
/// ```
/// let suggestions = suggest_similar_branches("mainn", &vec!["main".into(), "dev".into()]);
/// assert_eq!(suggestions, vec!["main"]);
/// ```
pub fn suggest_similar_branches(input: &str, candidates: &[String]) -> Vec<String> {
    let mut matches: Vec<_> = candidates
        .iter()
        .map(|c| (levenshtein(input, c), c))
        .filter(|(dist, _)| *dist <= 3)
        .collect();
    matches.sort_by_key(|k| k.0);
    matches.into_iter().map(|(_, s)| s.clone()).collect()
}
