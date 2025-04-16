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
Ahead Behind Branch                                                             Commit  Last Commit
----- ------ -------------------------------------------------------------------------- ------------
   29    275 origin/gh-pages                                                    dccd828 8 days ago
    0      0 main                                                               17d44f9 8 days ago
    0      0 origin                                                             17d44f9 8 days ago
    0      0 origin/main                                                        17d44f9 8 days ago
    0      1 origin/develop                                                     cf7cdda 8 days ago
    0      5 origin/chaange-file-location                                       0847de0 8 days ago
    0     14 origin/US000045-add-front-end-tests                                93ece51 8 days ago
    0     33 origin/backend-unit-tests                                          2ec685b 8 days ago
    0     42 origin/us000005-requirements-engineering-markdown                  4c641f0 8 days ago
    0     45 origin/final-changes                                               9f51f47 8 days ago
    0    102 origin/Create-Active-jobs-endpoint                                 c9e8a35 8 days ago
    0     59 origin/US000008-Create-systemModelling.md-and-complete-content-    b99713b 8 days ago
    0     55 origin/Delete-controller-used-for-testing                          5dec265 8 days ago
    0     47 origin/quick-fixes                                                 9a577ec 8 days ago
    0     49 origin/HOTFIX-adding-loading-when-users-try-to-log-in              2ceb786 8 days ago
    0     53 origin/update-home-screen                                          140909d 8 days ago
    0     55 origin/only-verified-users-can-access-dashboards                   7aa30c9 8 days ago
    0    256 origin/us000007-create-application-design-using-design-tool        3348f0d 9 days ago
    0     81 origin/us000044-fix-react-icon-being-on-every-job                  e2dd5d3 9 days ago
    0     82 origin/US000043-fix-password-being-returned-in-jobseekercontroller 4caff2c 9 days ago
    0     87 origin/US000041-Create-Employer-dashboard                          d734e19 12 days ago
    0     90 origin/US000029-set-up-API-connections-for-login-signup            6f7b8cc 3 weeks ago
    0     96 origin/us000039-testing-gcp-onfront-end                            c6aa254 3 weeks ago
    0    206 origin/update-application-properties                               0e9db11 7 weeks ago
    0    243 origin/US000018-Update-homepage-goals-css                          a944ec3 9 weeks ago
    5    245 origin/us000005-requirements-engineering                           23d382e 10 weeks ago
    0    252 origin/US000003-Create-React-App-in-Typescript                     77af51b 3 months ago
    0    260 origin/US000004-Create-Spring-Backend-App                          10a955d 3 months ago
    0    271 origin/US000002_Project_Structure_Setup                            a7ee4cd 3 months ago
    0    273 origin/feedback                                                    2734b2a 3 months ago
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
