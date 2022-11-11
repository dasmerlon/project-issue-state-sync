# Project Issue State Sync


## Usage

**ProjectV2**


## Inputs

### Token scopes
This must be a personal access token. \
Fine grained PATs **won't work**.
- repo
- read:org (if set up for an organization)
- read:project

### Creating a PAT and adding it to your repository


## Supported Events


## Example

https://crontab.guru/

```yaml
name: Project Issue State Sync

on: 
  workflow_dispatch:
  schedule: 
    # At minute 0 every 12th hour (0:00 and 12:00)
    - cron: 0 0/12 * * * 

jobs:
  issue-state-sync:
    runs-on: ubuntu-latest

    steps:
      - name: Sync issue states
        uses: dasmerlon/project-issue-state-sync@v0.1.0
        with:
          github_token: ${{ secrets.PROJECT_AUTO_ASSIGN_TOKEN }}
          owner: OWNER_NAME
          project_number: 1
          closed_stati: Done
          open_stati: Todo,In Progress

```

## Debugging