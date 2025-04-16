//! Data structure representing a Git branch's comparison status.
//!
//! This module defines a `BranchInfo` struct used for storing per-branch information
//! such as name, time of last commit, and how many commits it is ahead/behind
//! a comparison branch.

/// Stores metadata about a single Git branch in relation to a base branch.
///
/// This struct is used to populate the display table of branches in the tool's output.
#[derive(Debug)]
pub struct BranchInfo {
    /// The short name of the branch (e.g. `main`, `origin/dev`).
    pub name: String,

    /// A human-readable representation of the last commit time (e.g. `5 days ago`).
    pub relative_time: String,
    
    /// The last commit hash (be4791c696db01729d9ffb54ea69cef15c37c619).
    pub last_commit_hash: String,

    /// Number of commits this branch is ahead of the base branch.
    pub ahead: i32,

    /// Number of commits this branch is behind the base branch.
    pub behind: i32,
}