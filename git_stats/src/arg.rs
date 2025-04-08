use clap::Parser;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::fs;
use std::process::Command;

/// Command-line configuration options for analyzing Git contributions.
#[derive(Debug, Parser)]
#[command(name = "gitStats", about = "Analyze Git commit contributions per author")]
pub struct Config {
    /// Filter results by a specific author name (case-insensitive).
    #[arg(long)]
    pub author: Option<String>,

    /// Include all users (e.g., GitHub, bots) in the results.
    #[arg(long, default_value_t = false)]
    pub all: bool,

    /// Include merge commits in the analysis.
    #[arg(long, default_value_t = false)]
    pub merge: bool,

    /// Git branch to analyze. Defaults to the current branch if not specified.
    #[arg(long)]
    pub branch: Option<String>,

    /// Inline glob patterns used to exclude files or directories.
    #[arg(long, value_name = "GLOB", num_args = 0.., trailing_var_arg = true)]
    pub exclude: Vec<String>,

    /// Path to a file containing additional exclude patterns (one per line).
    #[arg(long, value_name = "FILE")]
    pub exclude_from_file: Option<String>,
}

impl Config {
    /// Determines the branch to analyze.
    ///
    /// Returns the user-supplied `--branch` value if provided,
    /// otherwise falls back to the currently checked-out Git branch.
    pub fn resolve_branch(&self) -> String {
        self.branch.clone().unwrap_or_else(|| {
            let output = Command::new("git")
                .args(["rev-parse", "--abbrev-ref", "HEAD"])
                .output()
                .expect("Failed to get current branch");
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        })
    }

    /// Collects and combines all ignore patterns from both CLI and file input.
    ///
    /// This includes:
    /// - Patterns passed using `--exclude`
    /// - Patterns read from the file passed via `--exclude-from-file`
    pub fn all_ignore_patterns(&self) -> Vec<String> {
        let mut combined = self.exclude.clone();
        if let Some(ref file) = self.exclude_from_file {
            if let Ok(content) = fs::read_to_string(file) {
                combined.extend(
                    content
                        .lines()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                );
            }
        }
        combined
    }

    /// Constructs a compiled `GlobSet` matcher from all ignore patterns.
    ///
    /// This is used to filter out files from commit stats.
    pub fn ignore_set(&self) -> GlobSet {
        let patterns = self.all_ignore_patterns();
        let mut builder = GlobSetBuilder::new();
        for pat in &patterns {
            if let Ok(glob) = Glob::new(pat) {
                builder.add(glob);
            }
        }
        builder.build().expect("Failed to build globset")
    }
}