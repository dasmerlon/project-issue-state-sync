# Project Issue State Sync

A GitHub Action that automatically closes or reopens issues if they are in a specific project column of a user- or organization-owned GitHub project _(ProjectV2)_. 

> **Note**  
> This action does not support the old [GitHub projects (classic)](https://docs.github.com/en/issues/organizing-your-work-with-project-boards).

## Usage

Github projects come with [built-in workflows](https://docs.github.com/en/issues/planning-and-tracking-with-projects/automating-your-project/using-the-built-in-automations) that allow you to automatically move an issue to a specific project column when it's closed or reopened. For example, an issue is automatically moved to `Done` when it's closed. Unfortunately, the other way around is not supported, so the issue stays open when you move it to `Done`. 

With this action you can specify the project columns in which issues should be closed or open. If an issue is in one of those project columns and has an undesired state, the action changes the issue state accordingly.

## Inputs

| Input variable   | Description                                                                                                                                                 | Required |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | :------: |
| `github_token`   | The personal access token                                                                                                                                   |     ✓    |
| `owner`          | The user or organization that owns the repository and project                                                                                               |     ✓    |
| `project_number` | The number of the project you target                                                                                                                        |     ✓    |   
| `closed_statuses`   | The project board column names in which issues should be closed <br> For example: `Won't do,Done` <br> _(Make sure there are no spaces between arguments)_  |     ✕    |   
| `open_statuses`     | The project board column names in which issues should be open <br> For example: `Todo,In Progress` <br> _(Make sure there are no spaces between arguments)_ |     ✕    |  
| `verbosity`      | The log output verbosity <br> Possible values: `info`, `debug`, `trace` <br> Default: `info`                                                                |     ✕    |

## Creating a PAT

Create a [personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token) with following scopes and add it to your repository as a [secret](https://docs.github.com/en/actions/security-guides/encrypted-secrets#creating-encrypted-secrets-for-a-repository). 

> **Note**  
> This must be a personal access token (classic).  
Fine grained PATs **won't work**.

### Token scopes

- [x] **repo**  
    - [x] repo:status
    - [x] repo_deployment
    - [x] public_repo
    - [x] repo:invite
    - [x] security_events
- [ ] **project**  
    - [x] read:project
- [ ] **admin:org**  
    - [ ] write:org
    - [x] read:org **_(only needed if set up for an organization)_**
    - [ ] manage_runners:org
    
## Creating a workflow

Create a [workflow](https://docs.github.com/en/actions/using-workflows) and save it as a `.yml` file in the `.github/workflows/` directory of your target repository.

### Supported Events

Unfortunately, GitHub only provides workflow trigger events for the older projects (classic). Therefore, there is no way to detect when an issue is moved to a different project column. 

However, you can use a scheduled workflow to periodically check if issues have been moved and correct their states retrospectively.

- The [workflow_dispatch](https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#workflow_dispatch) event allows you to manually trigger a workflow.

- The [schedule](https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#schedule) event allows you to trigger a workflow at a scheduled time.  
_If you need help formulating the cron schedule expression, you can use [crontab guru](https://crontab.guru/)._

### Example workflow

This workflow runs every 2 hours or if manually started.

```yaml
name: Project Issue State Sync

on: 
  schedule:
    # At minute 0 every 2 hours
    - cron: 0 0-23/2 * * *
  
  workflow_dispatch:
    # Manual trigger

jobs:
  issue-state-sync:
    runs-on: ubuntu-latest

    steps:
      - name: Sync issue states
        uses: dasmerlon/project-issue-state-sync@v1.0.0
        with:
          github_token: ${{ secrets.PROJECT_ISSUE_SYNC_TOKEN }}
          owner: OWNER_NAME
          project_number: 1
          closed_statuses: Done
          open_statuses: Todo,In Progress

```

