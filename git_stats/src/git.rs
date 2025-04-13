use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::{Command, exit};
use std::str;
use std::io::{self, Write};

use crate::arg::Config;
use crate::report::AuthorStats;

/// Checks whether we're inside a valid Git repository and git is available.
pub fn in_git_repo_check() {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                if result.trim() != "true" {
                    eprintln!("Error: Git is installed, but this is not inside a Git working tree.");
                    exit(1);
                }
            } else {
                eprintln!("Error: Git command failed. Are you sure this is a Git repository?");
                exit(1);
            }
        }
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("Error: Git is not installed or not found in PATH.");
                exit(1);
            }
            _ => {
                eprintln!("Error: Failed to execute Git command: {}", err);
                exit(1);
            }
        },
    }
}

/// Main function to collect contribution statistics by author.
pub fn collect_git_stats(config: &Config) -> HashMap<String, AuthorStats> {
    let ignore_set = config.ignore_set();
    let mut seen_hashes = HashSet::new();
    let mut stats: HashMap<String, AuthorStats> = HashMap::new();

    let branch = config.resolve_branch();
    let mut args = vec!["log", "--all", "--pretty=format:%H|%cn"]; // using --all to catch merges across all branches
    if !config.merge {
        args.push("--no-merges");
    }

    let output = Command::new("git")
        .args(&args)
        .output()
        .expect("Failed to run git log");

    let output_str = str::from_utf8(&output.stdout).unwrap_or("");
    let lines: Vec<&str> = output_str.lines().collect();
    let total = lines.len();

    for (i, line) in lines.iter().enumerate() {
        print!("\rProcessing commit {:>5}/{:<5}", i + 1, total);
        io::stdout().flush().unwrap();

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            continue;
        }

        let commit_hash = parts[0];
        if !seen_hashes.insert(commit_hash.to_string()) {
            continue;
        }

        let author = parts[1].to_lowercase().trim().to_string();

        if !config.all && (author.contains("github") || author.contains("bot")) {
            continue;
        }

        if !config.merge && is_merge_commit(commit_hash) {
            continue;
        }

        if let Some(author_filter) = &config.author {
            if &author != &author_filter.to_lowercase() {
                continue;
            }
        }

        let diff_output = Command::new("git")
            .args(["show", commit_hash, "--numstat", "--pretty=format:"])
            .output()
            .expect("Failed to run git show");

        let diff_str = str::from_utf8(&diff_output.stdout).unwrap_or("");
        let mut insertions = 0;
        let mut deletions = 0;
        let mut files = Vec::new();

        for line in diff_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue;
            }
            let file = parts.last().unwrap();
            let normalized_path = file.replace('\\', "/");
            let lowered_path = normalized_path.to_lowercase();
            let path = Path::new(&lowered_path);

            if ignore_set.is_match(path) {
                continue;
            }

            let ins = parts[0].parse::<u32>().unwrap_or(0);
            let del = parts[1].parse::<u32>().unwrap_or(0);
            insertions += ins;
            deletions += del;
            files.push(normalized_path);
        }

        if insertions + deletions == 0 && files.is_empty() {
            continue;
        }

        stats.entry(author)
            .or_default()
            .add_commit(commit_hash, files, insertions, deletions);
    }

    println!("
Done processing {} commits.", total);
    stats
}

/// Helper to determine if a commit is a merge (has >1 parent)
fn is_merge_commit(commit_hash: &str) -> bool {
    let output = Command::new("git")
        .args(["rev-list", "--parents", "-n", "1", commit_hash])
        .output()
        .expect("Failed to check merge commit");

    let line = str::from_utf8(&output.stdout).unwrap_or("");
    line.split_whitespace().count() > 2
}
