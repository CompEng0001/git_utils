mod git;
mod report;
mod arg;

use crate::git::{in_git_repo_check,collect_git_stats};
use crate::report::display_stats;
use clap::Parser;
use crate::arg::Config;

/// Entry point for the `gitStats` CLI application.
///
/// This function parses command-line arguments into a [`Config`] struct,
/// collects Git contribution statistics using the provided configuration,
/// and outputs a formatted report to the terminal.
fn main() {
    // Check to see if git exists and in a git repo
    in_git_repo_check();
    
    // Parse command-line arguments
    let config = Config::parse();

    // Collect Git statistics based on the configuration
    let stats = collect_git_stats(&config);

    // Display the summary report
    display_stats(stats);
}