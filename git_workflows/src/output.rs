//! Output utilities for user-friendly terminal messages.
//!
//! This module formats and displays GitHub API responses and general status messages
//! using consistent terminal output formatting and color for better readability.

use colored::*;
use serde_json::Value;


// Print only the colored header line (no rows yet).
pub fn print_tree_header(tag: &str, color: Color, header: &str) {
    let colored_tag = match color {
        Color::Blue => tag.blue().bold(),
        Color::Green => tag.green().bold(),
        Color::Yellow => tag.yellow().bold(),
        Color::Red => tag.red().bold(),
        _ => tag.normal(),
    };
    println!("{} {}", colored_tag, header);
}

// Print a single branch row under a previously-printed header.
// If `is_last` is true, uses └─; otherwise uses ├─.
pub fn print_tree_branch(tag: &str, is_last: bool, key: &str, val: &str) {
    let pad = " ".repeat(tag.len() + 1); // align under header after "Tag:"
    let branch = if is_last { "└─" } else { "├─" };
    println!("{pad}      {branch} {key}: {val}");
}


/// Prints a detailed GitHub API error message to stderr.
///
/// Attempts to parse the response body as JSON to extract a message and documentation URL.
/// Falls back to raw body display if parsing fails.
///
/// # Arguments
/// * `status` - HTTP status code
/// * `body` - API response body (expected JSON)
/// * `repo_owner` - GitHub username or organization
/// * `repo_name` - Repository name
pub fn print_github_api_error(status: u16, body: &str, repo_owner: &str, repo_name: &str) {
    eprintln!("{}", "GitHub API Error".red().bold());
    eprintln!("HTTP Status: {}", status);

    if let Ok(err_json) = serde_json::from_str::<Value>(body) {
        let msg = err_json.get("message").and_then(|v| v.as_str()).unwrap_or("No message");
        let doc = err_json.get("documentation_url").and_then(|v| v.as_str()).unwrap_or("No URL");
        eprintln!("Message: {}", msg);
        eprintln!(
            "Tip: Check that the repository '{}/{}' exists and your token has access.",
            repo_owner, repo_name
        );
        eprintln!("Docs: {}", doc);
    } else {
        eprintln!("Raw Response: {}", body);
    }
}

/// Prints a general info message with blue prefix.
pub fn print_info(msg: &str) {
    println!("{} {}", "Info:".blue().bold(), msg);
}

/// Prints a warning message with yellow prefix.
pub fn print_warning(msg: &str) {
    println!("{} {}", "Warning:".yellow().bold(), msg);
}