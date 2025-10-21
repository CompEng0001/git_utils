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
use crate::output::print_github_api_error;
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

    let mut v: Value;
    let mut workflow_name: &str;
    let mut status: &str;

    loop {
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/runs",
            repo_owner, repo_name
        );

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
                let status = resp.status();
                let body = resp.text().await.unwrap_or_else(|_| {
                    eprintln!("Failed to read response body");
                    exit(1);
                });

                if debug {
                    print_info(&format!("HTTP status: {}", status));
                }

                if !status.is_success() {
                    print_github_api_error(status.as_u16(), &body, repo_owner, repo_name);
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

        status = v["workflow_runs"][0]["status"].as_str().unwrap();
        workflow_name = v["workflow_runs"][0]["name"].as_str().unwrap();

        match status {
            "in_progress" | "queued" | "waiting" |"pending" | "requested" => print_info(&format!("Workflow[{}] | state: {}", workflow_name, status)),
            "completed" => {
                print_info(&format!("Workflow[{}] | state: {}", workflow_name, status));
                break;
            }
            _ => {
                print_info(&format!("Workflow[{}] | state: {}", workflow_name, status));
                break;
            }
        }

        sleep(Duration::from_secs(20)).await;
    }

    let conclusion = v["workflow_runs"][0]["conclusion"].as_str().unwrap();
    let start_time = v["workflow_runs"][0]["created_at"].as_str().unwrap();
    let end_time = v["workflow_runs"][0]["updated_at"].as_str().unwrap();

    let start_time = DateTime::parse_from_rfc3339(start_time).expect("Failed to parse start time");
    let end_time = DateTime::parse_from_rfc3339(end_time).expect("Failed to parse end time");
    let duration = end_time.signed_duration_since(start_time);

    if conclusion == "success"{
        print_success(&format!(
            "Workflow[{}] | conclusion: {} | Duration: {}s | Completed at: {}",
            workflow_name,
            conclusion,
            duration.num_seconds(),
            end_time
        ));
    }
    else {
        print_failure(&format!(
            "Workflow[{}] | conclusion: {} | Duration: {}s | Completed at: {}",
            workflow_name,
            conclusion,
            duration.num_seconds(),
            end_time
        ));
    }
    
    fs::remove_file(&workflow_file).expect("Unable to remove workflow file");
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