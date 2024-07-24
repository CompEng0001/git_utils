use std::collections::HashMap;
use std::process::Command;
use std::str;

#[derive(Default)]
struct AuthorStats {
    commits: u32,
    insertions: u32,
    deletions: u32,
}

impl AuthorStats {
    fn new() -> Self {
        AuthorStats {
            commits: 0,
            insertions: 0,
            deletions: 0,
        }
    }

    fn add_stats(&mut self, insertions: u32, deletions: u32) {
        self.commits += 1;
        self.insertions += insertions;
        self.deletions += deletions;
    }

    fn insertion_deletion(&self) -> u32 {
        (self.insertions as i32 - self.deletions as i32).abs() as u32
    }
}

fn main() {
    let branch = std::env::args().nth(1).unwrap_or("--all".to_string());
    let output = Command::new("git")
        .arg("log")
        .arg(branch)
        .arg("--shortstat")
        .arg("--pretty=format:%cn")
        .output()
        .expect("Failed to execute git command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert git output to string");

    let mut stats: HashMap<String, AuthorStats> = HashMap::new();
    let mut author = String::new();

    for line in output_str.lines() {
        if line.contains(" changed") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let insertions = parts.iter().position(|&s| s == "insertion(+)").or_else(|| parts.iter().position(|&s| s == "insertions(+)"))
                .map_or(0, |pos| parts[pos - 1].replace(",", "").parse::<u32>().unwrap_or(0));
            let deletions = parts.iter().position(|&s| s == "deletion(-)").or_else(|| parts.iter().position(|&s| s == "deletions(-)"))
                .map_or(0, |pos| parts[pos - 1].replace(",", "").parse::<u32>().unwrap_or(0));

            if let Some(author_stats) = stats.get_mut(&author) {
                author_stats.add_stats(insertions, deletions);
            }
        } else {
            author = line.to_lowercase().trim().to_string();
            if !author.is_empty() {
                stats.entry(author.clone()).or_insert(AuthorStats::new());
            }
        }
    }

    let author_width = stats.keys().map(|a| a.len()).max().unwrap_or(0);
    let author_width = author_width + 2; // Padding for better formatting

    println!("{:<width$}{:<10}{:<12}{:<12}{:<12}", "Author", "Commits", "Insertions", "Deletions", "Insertion-Deletion", width = author_width);

    let mut total_commits = 0;
    let mut total_insertions = 0;
    let mut total_deletions = 0;
    let mut total_i_d_diff = 0;

    for (author, stat) in &stats {
        let i_d_diff = stat.insertion_deletion();
        total_commits += stat.commits;
        total_insertions += stat.insertions;
        total_deletions += stat.deletions;
        total_i_d_diff += i_d_diff;

        println!(
            "{:<width$}{:<10}{:<12}{:<12}{:<12}",
            author,
            stat.commits,
            stat.insertions,
            stat.deletions,
            i_d_diff,
            width = author_width
        );
    }

    let author_count = stats.len() as f32;
    println!(
        "{:<width$}{:<10}{:<12}{:<12}{:<12}",
        "Total",
        total_commits,
        total_insertions,
        total_deletions,
        total_i_d_diff,
        width = author_width
    );
    println!(
        "{:<width$}{:<10.2}{:<12.2}{:<12.2}{:<12.2}",
        "Avg",
        total_commits as f32 / author_count,
        total_insertions as f32 / author_count,
        total_deletions as f32 / author_count,
        total_i_d_diff as f32 / author_count,
        width = author_width
    );
}
