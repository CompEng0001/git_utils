use clap::{Arg, Command};

/// Command-line arguments parsed from the user input.
pub struct CliArgs {
/// Maximum directory depth to search for Git repositories.
    ///
    /// A depth of 0 means only the current directory is scanned.
    pub depth: usize,

    /// Whether to show all branches (local and remote).
    ///
    /// If true, overrides `show_remote` and includes both refs/heads and refs/remotes.
    pub show_all: bool,

    /// Whether to show only remote branches.
    ///
    /// Ignored if `show_all` is true.
    pub show_remote: bool,

    /// Optional user-defined base branch to compare against.
    ///
    /// If `None`, the tool attempts to use `main`, then `master`, then `HEAD`.
    pub base_branch: Option<String>,
}

/// Parses command-line arguments using `clap` and returns a `CliArgs` struct.
///
/// # Behavior
///
/// - `--depth <N>` sets the maximum directory depth for scanning Git repos.
/// - `--all` shows both local and remote branches.
/// - `--remote` shows only remote branches.
/// - `--branch <BRANCH>` compares all branches to the specified branch.
///
/// If no flags are provided, the default behavior is:
/// - Show only local branches (`refs/heads/`)
/// - Use `main`, `master`, or `HEAD` as the comparison base
///
/// # Returns
///
/// A `CliArgs` struct containing the parsed arguments.
pub fn parse_args() -> CliArgs {
    let matches = Command::new("git-better-branch")
        .about("A better way to see your Git branches across projects")
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_name("DEPTH")
                .help("Sets the maximum directory depth to search, useful for multiple repositories in a parent directory")
                .num_args(1),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .help("Show all branches (local and remote)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("remote")
                .long("remote")
                .help("Show only remote branches")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("branch")
                .long("branch")
                .value_name("BRANCH")
                .help("Use this branch as the comparison base")
                .num_args(1),
        )
        .get_matches();

    CliArgs {
        depth: matches
            .get_one::<String>("depth")
            .and_then(|d| d.parse().ok())
            .unwrap_or(0),
        show_all: matches.get_flag("all"),
        show_remote: matches.get_flag("remote"),
        base_branch: matches.get_one::<String>("branch").cloned(),
    }
}
