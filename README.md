# git_utils 

Rust implementation of the Bash scripts I have written for various git utlities I use 

## git_workflow

Checks the current running/ran workflow, I mainly use this for checking the deployment of github pages.

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

```
Repo: git@github.com:USER/REPO
Ahead Behind Branch                         Last Commit
----- ------ ------------------------------ -------------------
   13      0 main                           7 weeks ago
    0      0 dev                            4 months ago
```

## git_stats

Summation and average of all author commits, insertions and deletions for a given repo or branch, I use this for ascertaining numerically the contribution each student has contributed to the repo. It is more of an indication, pinch of salt.

$ git_stats 

```
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

## Future

More utilities might be added if I need them. 

## Licence