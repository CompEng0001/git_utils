use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use walkdir::WalkDir;
use regex::Regex;

const RED: &str = "\x1b[0;31m";
const GREEN: &str = "\x1b[0;32m";
const NO_COLOR: &str = "\x1b[0m";
const BLUE: &str = "\x1b[0;34m";
const YELLOW: &str = "\x1b[0;33m";

fn count_commits(dir: &str, branch: &str, base_branch: &str) -> (i32, i32) {
    let output = Command::new("git")
        .args(&["-C", dir, "rev-list", "--left-right", "--count", &format!("{}...{}", base_branch, branch)])
        .output()
        .expect("Failed to execute git rev-list command");

    let ahead_behind = str::from_utf8(&output.stdout).unwrap().trim();
    let parts: Vec<&str> = ahead_behind.split('\t').collect();
    let behind = parts[0].parse().unwrap_or(0);
    let ahead = parts[1].parse().unwrap_or(0);
    (ahead, behind)
}

fn is_git_repo(path: &str) -> bool {
    Command::new("git")
        .args(&["-C", path, "rev-parse"])
        .stderr(Stdio::null())
        .status()
        .map_or(false, |status| status.success())
}

fn process_repo(dir: &str) {
    let repo_name = Command::new("git")
        .args(&["-C", dir, "remote", "get-url", "origin"])
        .output()
        .expect("Failed to get remote URL")
        .stdout;

    let repo_name = str::from_utf8(&repo_name)
        .unwrap()
        .trim()
        .replace(".git", "");

    println!("Repo: {}", repo_name);

    let main_branch = Command::new("git")
        .args(&["-C", dir, "rev-parse", "HEAD"])
        .output()
        .expect("Failed to get main branch")
        .stdout;

    let main_branch = str::from_utf8(&main_branch).unwrap().trim();

    println!(
        "{}{:5} {}{:6} {}{:30} {}{:20} {}{:40}",
        GREEN, "Ahead", RED, "Behind", BLUE, "Branch", YELLOW, "Last Commit", NO_COLOR, " "
    );
    println!(
        "{}{:5} {}{:6} {}{:30} {}{:20} {}{:40}",
        GREEN, "-----", RED, "------", BLUE, "------------------------------", YELLOW, "-------------------", NO_COLOR, " "
    );

    let branches_output = Command::new("git")
        .args(&["-C", dir, "for-each-ref", "--sort=-authordate", "--format=%(objectname:short)@%(refname:short)@%(committerdate:relative)", "refs/heads/"])
        .output()
        .expect("Failed to list branches")
        .stdout;

    let branch_output_str = str::from_utf8(&branches_output).expect("Invalid UTF-8 in branch output");
    let branch_regex = Regex::new(r"([^\@]+)@([^\@]+)@([^\@]+)").unwrap();
    let branches = branch_output_str.trim().lines();

    for branchdata in branches {
        if let Some(caps) = branch_regex.captures(branchdata) {
            let sha = &caps[1];
            let branch = &caps[2];
            let time = &caps[3];

            if branch != main_branch {
                let (ahead, behind) = count_commits(dir, sha, main_branch);
                println!(
                    "{}{:5} {}{:6} {}{:30} {}{:20} {}{:40}",
                    GREEN, ahead, RED, behind, BLUE, branch, YELLOW, time, NO_COLOR, ""
                );
            }
        }
    }
    println!();
}

fn check_all_dirs(path: &Path, depth: usize) {
    for entry in WalkDir::new(path)
        .min_depth(0)
        .max_depth(depth)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_dir() {
            if is_git_repo(path.to_str().unwrap()) {
                process_repo(path.to_str().unwrap());
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let depth = args.get(1).and_then(|d| d.parse().ok()).unwrap_or(0);
    let current_dir = env::current_dir().expect("Failed to get current directory");

    check_all_dirs(&current_dir, depth);
}
