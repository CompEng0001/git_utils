//! Entry point for the `git-better-branch` tool.
//!
//! This binary scans directories for Git repositories and prints out branch information
//! (local, remote, or both) in comparison to a reference branch (e.g., `main`, `master`, `HEAD`, or a user-defined branch).
//!
//! The CLI behavior is configured via arguments defined in `args.rs`.

mod git;
mod args;
mod report;
mod branch_info;

use walkdir::WalkDir;
use crate::args::parse_args;
use crate::git::is_git_repo;
use crate::report::process_repo_with_refs;

/// Main entry point of the program.
///
/// 1. Parses CLI arguments using `clap`.
/// 2. Determines which types of branches (local/remote/all) to scan.
/// 3. Walks the current directory (up to a specified depth) to find Git repositories.
/// 4. For each Git repository found, processes its branches for comparison.
///
/// If no repositories are found, prints an appropriate message.
///
/// # Panics
///
/// Will panic if the current working directory cannot be retrieved.
fn main() {
    let args = parse_args();

    let refs_arg = if args.show_all {
        vec!["refs/heads/", "refs/remotes/"]
    } else if args.show_remote {
        vec!["refs/remotes/"]
    } else {
        vec!["refs/heads/"]
    };

    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let mut found_repos = false;

    for entry in WalkDir::new(&current_dir)
        .min_depth(0)
        .max_depth(args.depth)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_dir() && is_git_repo(path.to_str().unwrap()) {
            found_repos = true;
            process_repo_with_refs(path.to_str().unwrap(), &refs_arg, args.base_branch.as_deref());
        }
    }

    if !found_repos {
        println!("No git repositories found in '{}'.", current_dir.display());
    }
}
