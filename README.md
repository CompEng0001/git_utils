# git_utils 

Rust implementation of the Bash scripts I have written for various git utlities I use.

## Platform

Built for Windows, will add Linux and MACOS soon. 

## git_workflow

Checks the current running/ran workflow, I mainly use this for checking the deployment of github pages.

You need to need to set an environment variable `GITHUB_TOKEN_PATH` that stores the path to your GitHub token. 

**For example:**

```sh
export GITHUB_TOKEN_PATH="/absolute/path/to/github_token
```

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

**Outputs:**

```
$ git_stats 

Author     Commits   Insertions  Deletions   Insertion-Deletion
author1       1         0           0           0
author2       3         101         0           101
author3       18        71          416         345
author4       154       2458        1736        722
author5       55        321         1947        1626
author6       17        198         204         6
author7       7         242         39          203
author8       2         218         0           218
author9       12        139         126         13
Total         269       3748        4468        3234
Avg           29.89     416.44      496.44      359.33
```

```
$git_stats dev

Author     Commits   Insertions  Deletions   Insertion-Deletion
author1       2         218         0           218
author2       3         101         0           101
author3       55        321         1947        1626        
author4       12        139         126         13
author5       7         242         39          203
author6       1         0           0           0
author7       16        70          412         342
author8       144       2409        1726        683
author9       17        198         204         6
Total         257       3698        4454        3192        
Avg           28.56     410.89      494.89      354.67 
```

## Git Tagging

I wanted functionality that auto increments tags for a workflow, where if a workflow sees the tag in the recent push then the github pages are deployed. 

Your Git commit message must be following syntax:

```
git commit -m "<add/del/fix/maj/modi>: message"
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
