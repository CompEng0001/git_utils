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
use crate::output::{print_github_api_error, print_tree_header, 
                    print_tree_branch, print_info, print_warning};

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

    // 1) Get the latest run ONCE and lock to its id.
    let client = reqwest::Client::new();
    let list_url = format!(
        "https://api.github.com/repos/{}/{}/actions/runs?per_page=1",
        repo_owner, repo_name
    );

    let list_resp = client
        .get(&list_url)
        .header(ACCEPT, "application/vnd.github.everest-preview+json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("token {}", token))
        .header(USER_AGENT, "Rust Workflow Check")
        .send()
        .await
        .map_err(|e| {
            print_warning(&format!(
                "Failed to request runs list for {}/{}: {}",
                repo_owner, repo_name, e
            ));
            e
        })
        .expect("GitHub API request failed");

    let http_status = list_resp.status();
    let body = list_resp.text().await.unwrap_or_else(|_| {
        eprintln!("Failed to read response body");
        exit(1);
    });

    if !http_status.is_success() {
        print_github_api_error(http_status.as_u16(), &body, repo_owner, repo_name);
        exit(1);
    }

    fs::write(&workflow_file, &body).ok();
    let mut v: Value = serde_json::from_str(&body).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON response: {}", e);
        eprintln!("Response body: {}", body);
        exit(1);
    });

    let run = &v["workflow_runs"][0];
    let run_id = run["id"].as_u64().expect("missing run id");
    let workflow_name = run["name"].as_str().unwrap_or("unknown").to_string();

    // 2) Print header once for this workflow, then poll /actions/runs/{id}.
    print_tree_header("Info:", colored::Color::Blue, &format!("Workflow[{}]", workflow_name));

    let mut last_status: Option<String> = None;

    loop {
        let run_url = format!(
            "https://api.github.com/repos/{}/{}/actions/runs/{}",
            repo_owner, repo_name, run_id
        );

        if debug {
            print_info(&format!("GET {}", run_url));
        }

        let resp = client
            .get(&run_url)
            .header(ACCEPT, "application/vnd.github.everest-preview+json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("token {}", token))
            .header(USER_AGENT, "Rust Workflow Check")
            .send()
            .await;

        let resp = match resp {
            Ok(r) => r,
            Err(e) => {
                print_warning(&format!(
                    "Failed to request run {} for {}/{}: {}",
                    run_id, repo_owner, repo_name, e
                ));
                exit(1);
            }
        };

        let status_code = resp.status();
        let body = resp.text().await.unwrap_or_else(|_| {
            eprintln!("Failed to read response body");
            exit(1);
        });

        if !status_code.is_success() {
            print_github_api_error(status_code.as_u16(), &body, repo_owner, repo_name);
            exit(1);
        }

        fs::write(&workflow_file, &body).ok();
        v = serde_json::from_str(&body).unwrap_or_else(|e| {
            eprintln!("Failed to parse JSON response: {}", e);
            eprintln!("Response body: {}", body);
            exit(1);
        });

        let status = v["status"].as_str().unwrap_or("unknown").to_string();

        // 3) Only print on state change. For the terminal state, print exactly once with └─.
        if last_status.as_deref() != Some(status.as_str()) {
            match status.as_str() {
                "completed" => {
                    print_tree_branch("Info:", true, "state", "completed");
                    println!();
                    break;
                }
                other => {
                    // mid-branch for non-terminal
                    print_tree_branch("Info:", false, "state", other);
                }
            }
            last_status = Some(status);
        }

        sleep(Duration::from_secs(poll_interval_secs)).await;
    }

    // 4) Final outcome (unchanged, but now uses /runs/{id} fields)
    let conclusion = v["conclusion"].as_str().unwrap_or("unknown");
    let start_time = v["created_at"].as_str().unwrap();
    let end_time = v["updated_at"].as_str().unwrap();

    let start_time = DateTime::parse_from_rfc3339(start_time).expect("Failed to parse start time");
    let end_time = DateTime::parse_from_rfc3339(end_time).expect("Failed to parse end time");
    let duration_secs = end_time.signed_duration_since(start_time).num_seconds();

    let rows = vec![
        ("conclusion".to_string(), conclusion.to_string()),
        ("duration".to_string(), format!("{}s", duration_secs)),
        ("completed at".to_string(), end_time.to_string()),
    ];

    if conclusion == "success" {
        print_tree_header("Success:", colored::Color::Green, &format!("Workflow[{}]", workflow_name));
        for (i, (k, v)) in rows.iter().enumerate() {
            print_tree_branch("Success:", i + 1 == rows.len(), k, v);
        }
        println!();
    } else {
        print_tree_header("Failure:", colored::Color::Red, &format!("Workflow[{}]", workflow_name));
        for (i, (k, v)) in rows.iter().enumerate() {
            print_tree_branch("Failure:", i + 1 == rows.len(), k, v);
        }
        println!();
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