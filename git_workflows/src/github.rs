//! GitHub API client for monitoring workflow runs and checking rate limits.
//!
//! This module provides functionality to:
//! - Poll the GitHub Actions API for the latest workflow run
//! - Print workflow state transitions (e.g., queued, in_progress, completed)
//! - Show final status and duration of the run
//! - Report the current GitHub API rate limit
//!
//! It relies on an authenticated GitHub token passed via the `Authorization` header.
use chrono::DateTime;
use serde_json::Value;
use std::{fs, process::exit, time::Duration};
use tokio::time::sleep;

use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use crate::output::{print_github_api_error, print_tree, print_tree_header, print_tree_branch};
use crate::output::{print_failure, print_success, print_info, print_warning};

/// Polls the GitHub Actions API for the most recent workflow run.
///
/// This function checks the status of the most recent GitHub Actions workflow run
/// for the given repository, polling periodically until the run is no longer active
/// (`queued`, `in_progress`, etc.). Once the run is complete, it prints:
/// - The workflow name
/// - Final state and conclusion
/// - Execution duration
///
/// It temporarily writes the API response to `.workflow.json` for debugging.
///
/// # Arguments
/// * `repo_owner` - The GitHub user/org that owns the repo.
/// * `repo_name` - The name of the repository.
/// * `token` - A valid GitHub API token.
/// * `debug` - prints the exact URL and headers you're sending for easier troubleshooting?
pub async fn monitor_workflow(
    repo_owner: &str,
    repo_name: &str,
    token: &str,
    debug: bool,
) {
    use std::collections::HashMap;

    let poll_interval_secs = 5u64;
    let workflow_file = std::env::current_dir()
        .expect("Unable to get current directory")
        .join(".workflow.json");

    if debug {
        print_info("Debug mode is ON");
        print_info(&format!("Workflow file: {:?}", workflow_file));
    }

    print_info(&format!(
        "Checking the last executed run in git@github.com:{}/{}...",
        repo_owner, repo_name
    ));
    println!();

    // Track per-workflow print state
    struct PrintState {
        header_printed: bool,
        last_status: Option<String>,
    }
    let mut printed: HashMap<String, PrintState> = HashMap::new();

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/runs",
        repo_owner, repo_name
    );

    let mut v: Value;

    loop {
        if debug {
            print_info(&format!("GET {}", url));
        }

        let response = client
            .get(&url)
            .header(ACCEPT, "application/vnd.github.everest-preview+json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("token {}", token))
            .header(USER_AGENT, "Rust Worflow Check")
            .send()
            .await;

        match response {
            Ok(resp) => {
                let http_status = resp.status();
                let body = resp.text().await.unwrap_or_else(|_| {
                    eprintln!("Failed to read response body");
                    exit(1);
                });

                if debug {
                    print_info(&format!("HTTP status: {}", http_status));
                }

                if !http_status.is_success() {
                    print_github_api_error(http_status.as_u16(), &body, repo_owner, repo_name);
                    exit(1);
                }

                fs::write(&workflow_file, &body).expect("Unable to write workflow file");

                v = serde_json::from_str(&body).unwrap_or_else(|e| {
                    eprintln!("Failed to parse JSON response: {}", e);
                    eprintln!("Response body: {}", body);
                    exit(1);
                });
            }
            Err(e) => {
                print_warning(&format!(
                    "Failed to send request to GitHub API for {}/{}: {}",
                    repo_owner, repo_name, e
                ));
                if debug {
                    print_info("Tip: Check internet access, proxy settings, or token validity.");
                }
                exit(1);
            }
        }

        // We’ll stream only the most recent run (index 0), as before.
        let workflow_name = v["workflow_runs"][0]["name"].as_str().unwrap_or("unknown").to_string();
        let status = v["workflow_runs"][0]["status"].as_str().unwrap_or("unknown").to_string();

        // Ensure header printed once
        let st = printed.entry(workflow_name.clone()).or_insert(PrintState {
            header_printed: false,
            last_status: None,
        });
        if !st.header_printed {
            print_tree_header("Info:", colored::Color::Blue, &format!("Workflow[{}]", workflow_name));
            st.header_printed = true;
        }

        // On a NEW state, print a branch line immediately.
        if st.last_status.as_deref() != Some(&status) {
            // While streaming, treat any non-terminal as a mid-branch ("├─").
            // We'll print the terminal ("└─") only when we reach `completed`.
            let is_last_for_now = false;
            print_tree_branch("Info:", is_last_for_now, "state", &status);
            st.last_status = Some(status.clone());
        }

        // Continue polling until terminal
        match status.as_str() {
            "in_progress" | "queued" | "waiting" | "pending" | "requested" => {
                tokio::time::sleep(Duration::from_secs(poll_interval_secs)).await;
                continue;
            }
            "completed" => {
                // Replace the last printed "mid" impression with an explicit terminal line.
                // We *also* print the final completed state as the last branch.
                print_tree_branch("Info:", true, "state", "completed");
                println!();
                break;
            }
            _ => {
                // Unknown or terminal-ish: stop polling
                print_tree_branch("Info:", true, "state", &status);
                println!();
                break;
            }
        }
    }

    // Final outcome block (printed immediately after completion)
    let conclusion = v["workflow_runs"][0]["conclusion"].as_str().unwrap_or("unknown");
    let name_for_outcome = v["workflow_runs"][0]["name"].as_str().unwrap_or("unknown");

    let start_time = v["workflow_runs"][0]["created_at"].as_str().unwrap();
    let end_time = v["workflow_runs"][0]["updated_at"].as_str().unwrap();

    let start_time = chrono::DateTime::parse_from_rfc3339(start_time).expect("Failed to parse start time");
    let end_time = chrono::DateTime::parse_from_rfc3339(end_time).expect("Failed to parse end time");
    let duration_secs = end_time.signed_duration_since(start_time).num_seconds();

    let rows = vec![
        ("conclusion".to_string(), conclusion.to_string()),
        ("duration".to_string(), format!("{}s", duration_secs)),
        ("completed at".to_string(), end_time.to_string()),
    ];

    if conclusion == "success" {
        print_tree_header("Success:", colored::Color::Green, &format!("Workflow[{}]", name_for_outcome));
        for (i, (k, v)) in rows.iter().enumerate() {
            print_tree_branch("Success:", i + 1 == rows.len(), k, v);
        }
    } else {
        print_tree_header("Failure:", colored::Color::Red, &format!("Workflow[{}]", name_for_outcome));
        for (i, (k, v)) in rows.iter().enumerate() {
            print_tree_branch("Failure:", i + 1 == rows.len(), k, v);
        }
    }

    fs::remove_file(&workflow_file).ok();
}

/// Checks the GitHub API rate limit using the `/rate_limit` endpoint.
///
/// Prints the number of remaining requests to stdout. This is useful for debugging
/// or verifying if your token is being throttled.
///
/// # Arguments
/// * `token` - A valid GitHub API token.
///
/// # Errors
/// Returns an error if the request fails or if the response headers can't be parsed.
pub async fn check_rate_limit(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/rate_limit")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "Rust Worflow Check")
        .header(AUTHORIZATION, format!("token {}", token))
        .send()
        .await?;

    let headers = response.headers();
    if let Some(rate_limit_remaining) = headers.get("X-RateLimit-Remaining") {
        let remaining = rate_limit_remaining.to_str()?.parse::<u32>()?;
        print_info(&format!("GitHub API rate limit remaining: {}", remaining));
    } else {
        print_warning("Could not find X-RateLimit-Remaining header.");
    }
    Ok(())
}