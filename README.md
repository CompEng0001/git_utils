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
$git_workflows
Info: Checking the last executed run in git@github.com:USER/REPO...
Workflow: pages build and deployment | state: waiting
Workflow: pages build and deployment | state: completed
Success: Workflow conclusion: success | Duration: 27s | Completed at: 2024-12-16 17:30:31 +00:00
Info: GitHub API rate limit remaining: 4976
Checking the last executed run in git@github.com:USER/REPO repository's workflow:
Workflow: deploy to github pages | state: queued
```

```

## git_better_branch

Checks current status of branches in repo, based off of the [better-branch.sh](https://gist.github.com/schacon/e9e743dee2e92db9a464619b99e94eff) script by [schacon](https://gist.github.com/schacon/).

```
$git_better-branch --help
A better way to see your Git branches across projects

Usage: git_better-branch.exe [OPTIONS]

Options:
  -d, --depth <DEPTH>    Sets the maximum directory depth to search
      --all              Show all branches (local and remote)
      --remote           Show only remote branches
      --branch <BRANCH>  Use this branch as the comparison base
  -h, --help             Print help
```

**Output:**

```
gitCheck --all
Repo: git@github.com:someuser/someuserepo
Comparing against: main
Ahead Behind Branch                                                 Last Commit
----- ------ ------------------------------------------------------ -----------
    0      0 main                                                   6 days ago
    0      0 origin                                                 6 days ago
    0      0 origin/main                                            6 days ago
    0     22 origin/develop                                         6 days ago
    0     23 origin/feature/US0038-accounts-styling-resolutions-fix 6 days ago
    0     28 origin/feature/US0037-Transactions-Pie-Chart           7 days ago
    0     79 origin/feature/US0036-dashboard-syling                 7 days ago
    0     54 origin/doc/ReadMe2                                     8 days ago
    0     51 origin/hc5167i-patch-1                                 9 days ago
    0    370 origin/doc/sprint-documents                            9 days ago
    0     82 origin/feature/U0036-Adjusting-GCP-Database            9 days ago
    0     85 origin/US0036/feature-fix                              9 days ago
    0     88 origin/feature/US0035-GCP-Revert                       9 days ago
    1    170 origin/feature/US0028-budgeting-page-styling           9 days ago
    24    568 origin/gh-pages                                        9 days ago
    8    157 origin/doc/ReadMe                                      10 days ago
    0    124 origin/feature/US0034-GCP-Connection                   3 weeks ago
    0    174 origin/feature/US0032-gh-pages                         4 weeks ago
    0    439 origin/doc/stand-up-documents                          4 weeks ago
    0    188 origin/feature/US0032-styling-updates                  5 weeks ago
    2    220 origin/feature/US0028-Budgeting-Frontend               5 weeks ago
    0    189 origin/testing/QA-Testing-2                            5 weeks ago
    0    202 origin/feature/US0030-Dashboard-Functionality          6 weeks ago
    0    204 origin/bug/US0031-Password-Verification-Fix            6 weeks ago
    0    210 origin/feature/US0027-accounts-page-styling            6 weeks ago
    0    212 origin/feature/US0029-QA-Fixes                         6 weeks ago
    0    419 origin/doc/requirements                                6 weeks ago
    0    234 origin/feature/US0023-transactions-page-styling        6 weeks ago
    0    234 origin/feature/US0024-transactions-page-styling        6 weeks ago
    0    234 origin/feature/US0025-Accounts-Backend                 7 weeks ago
    0    242 origin/feature/US0023-Transactions-Jar-Link            7 weeks ago
    0    281 origin/feature/US0022-Styling-Fixes-Opening-Pages      7 weeks ago
    0    291 origin/qa/user_test_home_login                         8 weeks ago
```

## git_stats

Summation and average of all author commits, insertions and deletions for a given repo or branch, I use this for ascertaining numerically the contribution each student has contributed to the repo. It is more of an indication, pinch of salt.

```md
$git_stats --help

Analyze Git commit contributions per author

Usage: git_stats.exe [OPTIONS]

Options:
      --author <AUTHOR>           Filter results by a specific author name (case-insensitive)
      --all                       Include all users (e.g., GitHub, bots) in the results
      --merge                     Include merge commits in the analysis
      --branch <BRANCH>           Git branch to analyze. Defaults to the current branch if not specified
      --exclude [<GLOB>...]       Inline glob patterns used to exclude files or directories
      --exclude-from-file <FILE>  Path to a file containing additional exclude patterns (one per line)
  -h, --help                      Print help
```

**Outputs:**

```
$ git_stats --all

Processing commit   455/455
Done processing 455 commits.

Author        Commits   Files         Insertions  Deletions   Net Change  Most Ext    Least Ext
author1       8         16            2036        350         1686        jsx         md
author2       10        14            787         240         547         md          py
author3       128       91            14654       10196       4458        css         txt
author4       24        66            4087        917         3170        jsx         css
author5       4         14            1199        123         1076        jsx         py
--------------------------------------------------------------------------------------------------
Total         174       201           22763       11826       10937       -           -
Avg           34.80     40.20         4552.60     2365.20     2187.40     -           -

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
