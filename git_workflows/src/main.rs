use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE,USER_AGENT};
use serde_json::Value;
use regex::Regex;
use std::{env, fs,path::PathBuf};
use chrono::{DateTime};
use tokio::time::{sleep, Duration};
use std::process::{Command,exit};

async fn get_repo_path(args: Vec<String>) -> (String, String) {
    let current_dir = env::current_dir().expect("Unable to get current directory");
    let output = Command::new("git")
        .current_dir(&current_dir)
        .args(&["rev-parse", "--is-inside-work-tree"])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true" {
        let config_path = current_dir.join(".git\\config");
        let config_content = fs::read_to_string(config_path).expect("Unable to read .git/config");
        let owner_re = Regex::new(r#"url\s*=\s*git@github\.com:(\w+)/([\w-]+)\.git"#).unwrap();
        let caps = owner_re.captures(&config_content).unwrap_or_else(|| {
            eprintln!("Failed to extract repository owner and name from .git/config");
            exit(1);
        });
        let owner = caps.get(1).unwrap().as_str().to_string();
        let name = caps.get(2).unwrap().as_str().to_string();
        (owner, name)

    } else if args.len() == 3 {
        (args[1].clone(), args[2].clone())
    } else {
        eprintln!("Position 1: Owner, position 2: repo name, or be in a git repo");
        exit(1);
    }
}

async fn check_rate_limit(github_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get("https://api.github.com/rate_limit")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "Rust Worflow Check")
        .header(AUTHORIZATION, format!("token {}", github_token))
        .send()
        .await?;

    let headers = response.headers();
    if let Some(rate_limit_remaining) = headers.get("X-RateLimit-Remaining") {
        let rate_limit_remaining = rate_limit_remaining.to_str()?.parse::<u32>()?;
        println!("API Rate Limit remaining: {}", rate_limit_remaining);
    } else {
        println!("Could not find X-RateLimit-Remaining header in the response.");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let (repo_owner, repo_name) = get_repo_path(args).await;

    // Read the GitHub token path from the environment variable
    let _env_var_github_token_path = env::var("GITHUB_TOKEN_PATH")
        .expect("GITHUB_TOKEN_PATH environment variable not set");

    let github_token_path = PathBuf::from(_env_var_github_token_path);
    
    // Check if the file exists
    if !github_token_path.exists() {
        eprintln!("GitHub token file does not exist at path: {:?}", github_token_path);
        exit(1);
    }

    let github_token = fs::read_to_string(&github_token_path)
        .expect("Unable to read GitHub token");

    let current_dir = env::current_dir().expect("Unable to get current directory");    
    let workflow_file = current_dir.join(".workflow.json");
    let sleep_interval = 20;

    println!("Checking the last executed run in git@github.com:{}/{} repository's workflow:", &repo_owner, &repo_name);

    let mut v: Value;

    loop {
        let client = reqwest::Client::new();
        let response = client.get(format!("https://api.github.com/repos/{}/{}/actions/runs", repo_owner, repo_name))
            .header(ACCEPT, "application/vnd.github.everest-preview+json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("token {}", github_token.trim()))
            .header(USER_AGENT, "Rust Worflow Check")
            .send()
            .await;

        match response {
            Ok(ref _resp) => {
                let body = response.expect("REASON").text().await.expect("Failed to read response body");
                fs::write(&workflow_file, &body).expect("Unable to write workflow file");

                v = serde_json::from_str(&body).expect("Failed to parse JSON");
            }
            Err(e) => {
                eprintln!("Failed to send request: {}", e);
                exit(1);
            }
        }

        let status = &v["workflow_runs"][0]["status"].as_str().unwrap();
        let workflow_name = &v["workflow_runs"][0]["name"].as_str().unwrap();

        match *status {
            "in_progress" | "queued" | "waiting" => println!("Workflow: {} | state: {}", &workflow_name, &status),
            "completed" => {
                println!("Workflow: {} | state: {}", &workflow_name, &status);
                break;
            },
            _ => {
                println!("Workflow: {} | state: {} | ❌", &workflow_name, &status);
                break;
            }
        }

        sleep(Duration::from_secs(sleep_interval)).await;
    }

    let conclusion = v["workflow_runs"][0]["conclusion"].as_str().unwrap();
    let start_time = v["workflow_runs"][0]["created_at"].as_str().unwrap();
    let end_time = v["workflow_runs"][0]["updated_at"].as_str().unwrap();

    let start_time = DateTime::parse_from_rfc3339(start_time).expect("Failed to parse start time");
    let end_time = DateTime::parse_from_rfc3339(end_time).expect("Failed to parse end time");

    let duration = end_time.signed_duration_since(start_time);
    println!("Workflow conclusion: {} | Time: {}s | DT: {}", conclusion, duration.num_seconds(), end_time);
    
    // Check the rate limit
    if let Err(e) = check_rate_limit(&github_token).await {
        eprintln!("Failed to check rate limit: {}", e);
    }
    
    fs::remove_file(workflow_file).expect("Unable to remove workflow file");
}