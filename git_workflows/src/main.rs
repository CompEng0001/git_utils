//! GitHub Workflow Monitor CLI
//!
//! This tool checks the most recent GitHub Actions workflow run for a repository,
//! either inferred from the local `.git/config` or provided explicitly via command-line arguments.
//!
//! It supports:
//! - Detecting the repository owner/name from the local git configuration
//! - Checking the workflow status via GitHub's API
//! - Displaying workflow name, state, duration, and conclusion
//! - Optional CLI overrides for `--owner` and `--repo`
//! - Reporting GitHub API rate limits
//!
//! # Example Usage
//!
//! ```bash
//! # Automatically detect the repo from the current directory
//! gw
//!
//! # Manually specify the GitHub repo
//! gw --owner your-username --repo your-repo
//! ```
mod args;
mod git;
mod github;
mod output;
mod token;

use args::Cli;
use git::get_repo_path;
use github::{check_rate_limit, monitor_workflow};
use token::get_github_token;

/// Entry point for the GitHub Workflow Monitor.
///
/// Uses command-line arguments (or Git config) to determine the target repository.
/// Reads the GitHub API token from the `GITHUB_TOKEN_PATH` environment variable.
/// Fetches the latest workflow run, polls it until complete, and prints the result.
#[tokio::main]
async fn main() {
    let cli = Cli::parse_args();

    // Prefer CLI args, fallback to Git config
    let (repo_owner, repo_name) = match (&cli.owner, &cli.repo) {
        (Some(owner), Some(repo)) => (owner.clone(), repo.clone()),
        _ => get_repo_path(vec![]).await,
    };

    let github_token = get_github_token();

    monitor_workflow(&repo_owner, &repo_name, &github_token, cli.debug).await;

    if let Err(e) = check_rate_limit(&github_token).await {
        eprintln!("Failed to check GitHub API rate limit.");
        eprintln!("Details: {}", e);
    }
}