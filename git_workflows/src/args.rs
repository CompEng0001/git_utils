//! Command-line argument parser for the GitHub Workflow Checker.
//!
//! This module defines the [`Cli`] struct and its parsing behavior using the [`clap`] crate.
//! It allows optional specification of the GitHub repository owner and name via `--owner` and `--repo` flags.
//!
//! If neither argument is supplied, the application falls back to detecting the repository from the local Git config.
use clap::Parser;

/// CLI interface for the GitHub Workflow Checker.
///
/// Supports optional arguments to manually specify the GitHub repository:
/// - `--owner`: GitHub username or organization (e.g., `uniofgreenwich`)
/// - `--repo`: Repository name (e.g., `ELEE1149_Exercises`)
///
/// If not provided, the tool attempts to extract this info from `.git/config`.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Repository owner (e.g. uniofgreenwich)
    #[arg(short, long)]
    pub owner: Option<String>,

    /// Repository name (e.g. ELEE1149_Exercises)
    #[arg(short, long)]
    pub repo: Option<String>,

    /// Enable debug mode (prints internal info)
    #[arg(long)]
    pub debug: bool,
}

impl Cli {
    /// Parse the CLI arguments and return a [`Cli`] instance.
    ///
    /// # Panics
    /// This will panic if argument parsing fails (e.g., invalid input format).
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}