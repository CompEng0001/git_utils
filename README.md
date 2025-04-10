<div align="center">
	<h1 align="center"><b>git_utils</b></h1>
</div>

<p align="center">
    <img src="https://img.shields.io/badge/Made%20with-rust-CE412B?style=for-the-badge&logo=rust&logoColor=white", alt="Made with Rust">
    <a href="https://github.com/CompEng0001/git_utils/blob/main/LICENSE.md">
        <img alt="GitHub License" src="https://img.shields.io/github/license/CompEng0001/git_utils?style=for-the-badge", alt="License MIT">
    </a>
    <img src="https://img.shields.io/github/v/release/compeng0001/git_utils?style=for-the-badge", alt="Release">
    <img src="https://img.shields.io/github/v/tag/compeng0001/git_utils?style=for-the-badge", alt="Tags"> 
  <a href="https://github.com/CompEng0001/git_utils/stargazers">
        <img src="https://img.shields.io/github/stars/CompEng0001/git_utils?style=for-the-badge" alt="GitHub Stars">
  </a>
</p>

Rust implementation of the Bash scripts I have written for various git utilities I use.

## Platform

Built for Windows, will add Linux and MACOS soon.

## git_workflow

Checks the current running/ran workflow, I mainly use this for checking the deployment of GitHub pages.

> [!IMPORTANT]
> You need to set an environment variable `GITHUB_TOKEN_PATH` that stores the path to your GitHub token.
>
> **For example:**
>
> ```sh
> export GITHUB_TOKEN_PATH="/absolute/path/to/github_token
> ```

**Output:**

```
Checking the last executed run in git@github.com:USER/REPO repository's workflow:
Workflow: deploy to github pages | state: queued
Workflow: pages build and deployment | state: waiting
Workflow: pages build and deployment | state: completed
Workflow: deploy to github pages | state: completed
Workflow conclusion: success | Time: 15s | DT: 2024-07-24 10:55:40 +00:00
API Rate Limit remaining: 4994
```

## git_better_branch

Checks current status of branches in repo, based off of the [better-branch.sh](https://gist.github.com/schacon/e9e743dee2e92db9a464619b99e94eff) script by [schacon](https://gist.github.com/schacon/).

**Output:**

```
Repo: git@github.com:USER/REPO
Ahead Behind Branch                         Last Commit
----- ------ ------------------------------ -------------------
   13      0 main                           7 weeks ago
    0      0 dev                            4 months ago
```

## git_stats

Summation and average of all author commits, insertions and deletions for a given repo or branch, I use this for ascertaining numerically the contribution each student has contributed to the repo. It is more of an indication, pinch of salt.

```md
$git_stats --help

Analyze Git commit contributions per author

Usage: git_stats.exe [OPTIONS]

Options:
--author <AUTHOR> Filter results by a specific author name (case-insensitive)
--all Include all users (e.g., GitHub, bots) in the results
--merge Include merge commits in the analysis
--branch <BRANCH> Git branch to analyze. Defaults to the current branch if not specified
--exclude [<GLOB>...] Inline glob patterns used to exclude files or directories
--exclude-from-file <FILE> Path to a file containing additional exclude patterns (one per line)
-h, --help Print help
```

**Outputs:**

```
$ git_stats --all

Author        Commits   Files         Insertions  Deletions   Net Change  Most Ext    Least Ext
github        5         6             71          1           70          md          py
author1       8         8             373         203         170         py          md
author2       2         3             69          1           68          md          py
--------------------------------------------------------------------------------------------------
Total         15        17            513         205         308         -           -
Avg           5.00      5.67          171.00      68.33       102.67      -           -

```

```
$git_Stats --branch dev
Processing commit    18/18
Done processing 18 commits.

Author        Commits   Files         Insertions  Deletions   Net Change  Most Ext    Least Ext
author1       8         8             373         203         170         py          md
author2       2         3             69          1           68          md          py
--------------------------------------------------------------------------------------------------
Total         10        11            442         204         238         -           -
Avg           5.00      5.50          221.00      102.00      119.00      -           -
```

## Git Tagging

I wanted functionality that auto increments tags for a workflow, where if a workflow sees the tag in the recent push then the github pages are deployed.

Your Git commit message must be following syntax:

```
git commit -m "<add/del/fix/maj/mod>: message"
```

Where `maj` is the key word to increment the major number and reset the minor and patch numbers, `add`, `mod` and `del` are consider minor, finally, `fix` is a patch:

```
tag v1.0.0 #maj.minor.patch
```

Where no tag currently exists `v1.0.0` will be generated

```
$ git_tagging
New Tag: v1.0.0 on Commit: 4e7091c
v1.0.0          Commit hash: 4e7091c

$ git log
* 4e7091c (HEAD -> main, tag: v1.0.0) add: added some files
* 5c5b09d init: initial commit

$ gcm "del: deleted an unused file"
[main 7348470] del: deleted an unused file
 1 file changed, 0 insertions(+), 0 deletions(-)
 delete mode 100644 second

$ git_tagging
New Tag: v1.1.0 on Commit: 7348470
v1.1.0          Commit hash: 7348470
v1.0.0          Commit hash: 4e7091c
```

## Future

More utilities might be added if I need them.
