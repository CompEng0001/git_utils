//! Reporting utilities for the `git-better-branch` tool.
//!
//! This module handles output formatting and Git branch comparison logic,
//! including dynamic table printing and validation of user-specified base branches.

use crate::git::{
    run_command_and_trim,
    count_commits,
    get_default_base_branch,
    list_all_branch_names,
    suggest_similar_branches,
};
use crate::branch_info::BranchInfo;
use regex::Regex;
use std::collections::HashSet;

const RED: &str = "\x1b[0;31m";
const GREEN: &str = "\x1b[0;32m";
const NO_COLOR: &str = "\x1b[0m";
const BLUE: &str = "\x1b[0;34m";
const YELLOW: &str = "\x1b[0;33m";

/// Processes a Git repository by listing its branches and comparing them to a reference branch.
///
/// This function:
/// - Determines the base comparison branch (from user input or default logic).
/// - Validates the user-specified branch, if given, and suggests similar ones if missing.
/// - Collects local and/or remote branches to compare.
/// - Computes ahead/behind commit counts relative to the base branch.
/// - Prints a dynamic, colorized summary table.
///
/// # Arguments
///
/// * `dir` - Path to the Git repository.
/// * `refs` - A list of ref prefixes to include (e.g., `refs/heads/`, `refs/remotes/`).
/// * `user_branch` - Optional user-specified branch to compare against.
pub fn process_repo_with_refs(dir: &str, refs: &[&str], user_branch: Option<&str>) {
    let repo_name = run_command_and_trim(&["-C", dir, "remote", "get-url", "origin"])
        .replace(".git", "");
    println!("Repo: {}", repo_name);

    let base_branch = if let Some(user_branch) = user_branch {
        let all_branches = list_all_branch_names(dir);
        if !all_branches.contains(&user_branch.to_string()) {
            eprintln!("{}Error:{} branch '{}' not found.", RED, NO_COLOR, user_branch);

            let suggestions = suggest_similar_branches(user_branch, &all_branches);
            if !suggestions.is_empty() {
                eprintln!("{}Did you mean:{}\n  - {}", YELLOW, NO_COLOR, suggestions.join("\n  - "));
            } else {
                eprintln!("No similar branches found.");
            }

            return;
        }
        user_branch.to_string()
    } else {
        get_default_base_branch(dir).unwrap_or_else(|| {
            run_command_and_trim(&["-C", dir, "rev-parse", "HEAD"])
        })
    };

    let is_sha = base_branch.len() == 40;
    println!("{}Comparing against:{} {}", YELLOW, NO_COLOR, base_branch);

    let mut args = vec![
        "-C", dir,
        "for-each-ref",
        "--sort=-authordate",
        "--format=%(objectname:short)@%(refname:short)@%(committerdate:relative)",
    ];
    args.extend(refs.iter().copied());

    let branches_output = run_command_and_trim(&args);
    let branch_regex = Regex::new(r"([^@]+)@([^@]+)@([^@]+)").unwrap();
    let mut seen = HashSet::new();
    let mut branches = Vec::new();

    for branchdata in branches_output.trim().lines() {
        if let Some(caps) = branch_regex.captures(branchdata) {
            let sha = &caps[1];
            let branch = &caps[2];
            let time = &caps[3];

            if branch == "origin/HEAD" || seen.contains(branch) {
                continue;
            }
            seen.insert(branch.to_string());

            let base = if is_sha { sha } else { branch };
            let (ahead, behind) = count_commits(dir, base, &base_branch);

            branches.push(BranchInfo {
                name: branch.to_string(),
                relative_time: time.to_string(),
                ahead,
                behind,
            });
        }
    }

    print_branch_table(&branches);
}

/// Prints a colorized, aligned table of Git branches and their status relative to a base.
///
/// The table includes the number of commits each branch is ahead/behind, the branch name,
/// and the relative timestamp of the last commit.
///
/// # Arguments
///
/// * `branches` - A slice of `BranchInfo` structures to display.
pub fn print_branch_table(branches: &[BranchInfo]) {
    let mut max_ahead = "Ahead".len();
    let mut max_behind = "Behind".len();
    let mut max_name = "Branch".len();
    let mut max_time = "Last Commit".len();

    for b in branches {
        max_ahead = max_ahead.max(b.ahead.to_string().len());
        max_behind = max_behind.max(b.behind.to_string().len());
        max_name = max_name.max(b.name.len());
        max_time = max_time.max(b.relative_time.len());
    }

    println!(
        "{}{:>width_a$}{} {}{:>width_b$}{} {}{:<width_n$}{} {}{:<width_t$}{}",
        GREEN, "Ahead", NO_COLOR,
        RED, "Behind", NO_COLOR,
        BLUE, "Branch", NO_COLOR,
        YELLOW, "Last Commit", NO_COLOR,
        width_a = max_ahead,
        width_b = max_behind,
        width_n = max_name,
        width_t = max_time,
    );

    println!(
        "{}{:->width_a$}{} {}{:->width_b$}{} {}{:->width_n$}{} {}{:->width_t$}{}",
        GREEN, "", NO_COLOR,
        RED, "", NO_COLOR,
        BLUE, "", NO_COLOR,
        YELLOW, "", NO_COLOR,
        width_a = max_ahead,
        width_b = max_behind,
        width_n = max_name,
        width_t = max_time,
    );

    for b in branches {
        println!(
            "{}{:>width_a$}{} {}{:>width_b$}{} {}{:<width_n$}{} {}{:<width_t$}{}",
            GREEN, b.ahead, NO_COLOR,
            RED, b.behind, NO_COLOR,
            BLUE, b.name, NO_COLOR,
            YELLOW, b.relative_time, NO_COLOR,
            width_a = max_ahead,
            width_b = max_behind,
            width_n = max_name,
            width_t = max_time,
        );
    }

    println!();
}
