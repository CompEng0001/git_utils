use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::Command;
use std::str;
use std::io::Write;

use crate::arg::Config;
use crate::report::AuthorStats;

/// Collects Git contribution statistics per author based on the provided configuration.
///
/// This function analyzes Git commit history on a specified branch, filters commits
/// according to author and merge criteria, and aggregates stats such as number of commits,
/// insertions, deletions, and unique files changed per author.
///
/// # Arguments
///
/// * `config` - A reference to the CLI configuration containing analysis options.
///
/// # Returns
///
/// A `HashMap` where the key is the author's name and the value is their aggregated statistics.
pub fn collect_git_stats(config: &Config) -> HashMap<String, AuthorStats> {
    let ignore_set = config.ignore_set();
    let mut seen_hashes = HashSet::new();
    let mut stats: HashMap<String, AuthorStats> = HashMap::new();

    let output = Command::new("git")
        .args(["log", &config.resolve_branch(), "--pretty=format:%H|%cn"])
        .output()
        .expect("Failed to run git log");

    let output_str = str::from_utf8(&output.stdout).unwrap();
    let lines: Vec<&str> = output_str.lines().collect();
    let total = lines.len();
    let mut count = 0;

    for line in lines {
        count += 1;
        print!("\rProcessing commit {:>5}/{:<5}      ", count, total);
        std::io::stdout().flush().unwrap();

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            continue;
        }

        let commit_hash = parts[0];
        if !seen_hashes.insert(commit_hash.to_string()) {
            continue; // skip duplicate commits
        }

        let author = parts[1].to_lowercase().trim().to_string();

        if !config.all && (author.contains("github") || author.contains("bot")) {
            continue; // skip GitHub/bot users unless --all is specified
        }

        if !config.merge && is_merge_commit(commit_hash) {
            continue; // skip merge commits unless --merge is specified
        }

        if let Some(author_filter) = &config.author {
            if &author != &author_filter.to_lowercase() {
                continue; // skip if this author doesn't match the filter
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
            if parts.len() == 3 {
                let file = parts[2];
                let normalized_path = file.replace('\\', "/");
                let lowered_path = normalized_path.to_lowercase();
                let path = Path::new(&lowered_path);

                if ignore_set.is_match(path) {
                    continue; // skip ignored files
                }

                let ins = parts[0].parse::<u32>().unwrap_or(0);
                let del = parts[1].parse::<u32>().unwrap_or(0);
                insertions += ins;
                deletions += del;
                files.push(normalized_path);
            }
        }

        if insertions + deletions == 0 && files.is_empty() {
            continue; // skip empty commits
        }

        stats.entry(author)
            .or_default()
            .add_commit(commit_hash, files, insertions, deletions);
    }

    println!("\nDone processing {} commits.", total);
    stats
}

/// Determines whether a commit is a merge commit.
///
/// A merge commit has more than one parent in the Git history.
///
/// # Arguments
///
/// * `commit_hash` - SHA of the commit to check.
///
/// # Returns
///
/// `true` if the commit is a merge commit, `false` otherwise.
fn is_merge_commit(commit_hash: &str) -> bool {
    let output = Command::new("git")
        .args(["rev-list", "--parents", "-n", "1", commit_hash])
        .output()
        .expect("Failed to check merge commit");

    let line = str::from_utf8(&output.stdout).unwrap_or("");
    line.split_whitespace().count() > 2
}
