// report.rs
use std::collections::HashMap;
use itertools::Itertools;

/// Stores contribution statistics for a single Git author.
#[derive(Default)]
pub struct AuthorStats {
    /// Set of unique commit hashes attributed to this author.
    pub commits: std::collections::HashSet<String>,
    /// Set of unique file paths modified by this author.
    pub files_modified: std::collections::HashSet<String>,
    /// File extension frequencies for the author's changes.
    pub extensions: HashMap<String, u32>,
    /// Total number of lines inserted by this author.
    pub insertions: u32,
    /// Total number of lines deleted by this author.
    pub deletions: u32,
}

impl AuthorStats {
    /// Records a commit in the author's statistics.
    ///
    /// If the commit hash has not already been recorded, its associated changes
    /// (insertions, deletions, and file modifications) will be added.
    ///
    /// # Arguments
    ///
    /// * `commit_hash` - The SHA of the commit.
    /// * `files` - A list of file paths modified in the commit.
    /// * `insertions` - Number of lines added in the commit.
    /// * `deletions` - Number of lines removed in the commit.
    pub fn add_commit(&mut self, commit_hash: &str, files: Vec<String>, insertions: u32, deletions: u32) {
        if self.commits.insert(commit_hash.to_string()) {
            self.insertions += insertions;
            self.deletions += deletions;
            for file in files {
                if self.files_modified.insert(file.clone()) {
                    if let Some(ext) = file.split('.').last() {
                        *self.extensions.entry(ext.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    /// Returns the absolute value of the net line change.
    ///
    /// This is calculated as the absolute difference between insertions and deletions.
    pub fn insertion_deletion(&self) -> u32 {
        (self.insertions as i32 - self.deletions as i32).abs() as u32
    }

    /// Returns the total number of unique commits attributed to the author.
    pub fn commit_count(&self) -> usize {
        self.commits.len()
    }

    /// Returns the most common file extension modified by this author.
    pub fn most_common_extension(&self) -> Option<String> {
        self.extensions.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(ext, _)| ext.clone())
    }

    /// Returns the least common file extension modified by this author.
    pub fn least_common_extension(&self) -> Option<String> {
        self.extensions.iter()
            .min_by_key(|(_, count)| *count)
            .map(|(ext, _)| ext.clone())
    }
}

/// Outputs a formatted summary table of all authors' contribution statistics.
///
/// This includes commit counts, files modified, line insertions/deletions,
/// net line changes, and most/least common file extensions. It also displays total
/// and average metrics.
///
/// # Arguments
///
/// * `stats` - A map of author names to their associated contribution stats.
pub fn display_stats(stats: HashMap<String, AuthorStats>) {
    let author_width = stats.keys().map(|a| a.len()).max().unwrap_or(0) + 2;

    println!("\n{:<width$}{:<10}{:<14}{:<12}{:<12}{:<12}{:<12}{:<12}", "Author", "Commits", "Files", "Insertions", "Deletions", "Net Change", "Most Ext", "Least Ext", width = author_width);

    let mut total_commits = 0;
    let mut total_files = 0;
    let mut total_insertions = 0;
    let mut total_deletions = 0;
    let mut total_net = 0;

    for (author, stat) in stats.iter().collect::<Vec<_>>().into_iter().sorted_by_key(|(a, _)| a.to_string()) {
        let files_modified = stat.files_modified.len();
        let net = stat.insertion_deletion();
        let commit_count = stat.commit_count();
        let most_ext = stat.most_common_extension().unwrap_or_else(|| "-".to_string());
        let least_ext = stat.least_common_extension().unwrap_or_else(|| "-".to_string());

        println!(
            "{:<width$}{:<10}{:<14}{:<12}{:<12}{:<12}{:<12}{:<12}",
            author,
            commit_count,
            files_modified,
            stat.insertions,
            stat.deletions,
            net,
            most_ext,
            least_ext,
            width = author_width
        );

        total_commits += commit_count;
        total_files += files_modified;
        total_insertions += stat.insertions;
        total_deletions += stat.deletions;
        total_net += net;
    }

    let author_count = stats.len() as f32;
    println!("{:-<1$}", "", author_width + 84);
    println!(
        "{:<width$}{:<10}{:<14}{:<12}{:<12}{:<12}{:<12}{:<12}",
        "Total",
        total_commits,
        total_files,
        total_insertions,
        total_deletions,
        total_net,
        "-",
        "-",
        width = author_width
    );
    println!(
        "{:<width$}{:<10.2}{:<14.2}{:<12.2}{:<12.2}{:<12.2}{:<12}{:<12}",
        "Avg",
        total_commits as f32 / author_count,
        total_files as f32 / author_count,
        total_insertions as f32 / author_count,
        total_deletions as f32 / author_count,
        total_net as f32 / author_count,
        "-",
        "-",
        width = author_width
    );
}