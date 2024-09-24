use std::process::Command;
use std::str;
use regex::Regex;
use std::io::{self, Write};

fn get_commit_message() -> Option<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--format=%B")
        .arg("-n")
        .arg("1")
        .arg("HEAD")
        .output()
        .expect("Failed to execute git log command");

    if output.status.success() {
        let commit_message = str::from_utf8(&output.stdout).unwrap().trim().to_string();
        Some(commit_message)
    } else {
        None
    }
}

fn get_latest_tag() -> Option<String> {
    let output = Command::new("git")
        .arg("describe")
        .arg("--tags")
        .arg("--abbrev=0")
        .output()
        .expect("Failed to execute git describe command");

    if output.status.success() {
        let latest_tag = str::from_utf8(&output.stdout).unwrap().trim().to_string();
        Some(latest_tag)
    } else {
        None
    }
}

fn create_version_tag() {
    let new_tag: String;
    
    // Check if there are any existing tags
    if let Some(latest_tag) = get_latest_tag() {
        // Parse the version tag (e.g., "v1.0.0")
        let re = Regex::new(r"^v(\d+)\.(\d+)\.(\d+)$").unwrap();
        if let Some(captures) = re.captures(&latest_tag) {
            let mut major: u32 = captures[1].parse().unwrap();
            let mut minor: u32 = captures[2].parse().unwrap();
            let mut patch: u32 = captures[3].parse().unwrap();
            
            // Get the latest commit message
            if let Some(commit_message) = get_commit_message() {
                let keyword_re = Regex::new(r"^(mod|add|del|fix|maj):").unwrap();
                if let Some(keyword_capture) = keyword_re.captures(&commit_message) {
                    let keyword = &keyword_capture[1];
                    
                    // Increment the version based on the keyword
                    match keyword {
                        "maj" => {
                            major += 1;
                            minor = 0;
                            patch = 0;
                        }
                        "mod" | "add" | "del" => {
                            minor += 1;
                        }
                        "fix" => {
                            patch += 1;
                        }
                        _ => {
                            println!("Unrecognized commit message keyword");
                            return;
                        }
                    }

                    // Create the new version tag
                    new_tag = format!("v{}.{}.{}", major, minor, patch);
                } else {
                    println!("Commit message format not recognized. Please use 'keyword: message'.");
                    return;
                }
            } else {
                println!("Failed to retrieve commit message.");
                return;
            }
        } else {
            println!("Failed to parse the latest tag.");
            return;
        }
    } else {
        // No tags exist, start with initial version
        new_tag = "v1.0.0".to_string();
    }

    // Get the commit hash
    let commit_hash_output = Command::new("git")
        .arg("rev-parse")
        .arg("--short=7")
        .arg("HEAD")
        .output()
        .expect("Failed to get commit hash");

    if !commit_hash_output.status.success() {
        println!("Failed to retrieve commit hash.");
        return;
    }
    
    let commit_hash = str::from_utf8(&commit_hash_output.stdout).unwrap().trim();

    // Create and push the new tag
    Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(&new_tag)
        .arg("-m")
        .arg(&format!("Commit hash: {}", commit_hash))
        .output()
        .expect("Failed to create the new tag");

    println!("New Tag: {} on Commit: {}", &new_tag, commit_hash);

    Command::new("git")
        .arg("push")
        .output()
        .expect("Failed to push tags");

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(&new_tag)
        .output()
        .expect("Failed to push the new tag to origin");

    // List the tags
    let tag_output = Command::new("git")
        .arg("tag")
        .arg("-n")
        .arg("--sort=-v:refname")
        .output()
        .expect("Failed to list tags");

    if tag_output.status.success() {
        let tags = str::from_utf8(&tag_output.stdout).unwrap();
        print!("{}", tags);
    } else {
        println!("Failed to retrieve tags.");
    }
}

fn main() {
    create_version_tag();
}
