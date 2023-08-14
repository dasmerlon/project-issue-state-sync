use anyhow::{Ok, Result};
use clap::Parser;
use log::{error, info, trace};
use octocrab::Octocrab;
use serde_json::json;

mod args;
mod raw_response;
mod response;

use args::Args;
use raw_response::Response;
use response::{Field, Item, PageInfo, Project};
use simplelog::{Config, LevelFilter, SimpleLogger};

use crate::{args::LogLevel, response::IssueState};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Set the verbosity level of the logger.
    let level = match args.verbosity {
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    };
    SimpleLogger::init(level, Config::default()).unwrap();

    info!(
        "closed_statuses: {:?}, open_statuses: {:?}",
        args.closed_statuses, args.open_statuses
    );

    // Inform the user if neither closed_statuses nor open_statuses is specified.
    if args.closed_statuses.is_empty() && args.open_statuses.is_empty() {
        error!("No project board column names were specified.");
        std::process::exit(1);
    }

    // Initialize the Github API client.
    let client = octocrab::Octocrab::builder()
        .personal_token(args.github_token.clone())
        .build()?;

    let mut end_cursor = None;
    let mut has_next_page = true;

    while has_next_page {
        info!("Processing issue batch.");
        PageInfo {
            end_cursor,
            has_next_page,
        } = process_issue_batch(&client, &args, &end_cursor).await?;
    }

    info!("All done.");
    Ok(())
}

/// Change the issue state if the issue's item is in one of the target columns
/// and the issue has the wrong state.
async fn ensure_issue_state(
    item: &Item,
    issue_state: IssueState,
    args: &Args,
    client: &Octocrab,
    option_ids: &Vec<String>,
) -> Result<()> {
    if option_ids.is_empty() {
        return Ok(());
    }

    // Ignore the item if it isn't in one of the target columns.
    let is_in_target_column = item
        .field_values
        .iter()
        .any(|field_value| option_ids.contains(&field_value.option_id));

    if !is_in_target_column {
        return Ok(());
    }

    // Ignore the issue if it already has the desired state.
    if issue_state == item.issue.state {
        return Ok(());
    }

    info!(
        "Found issue #{} '{}' in column '{}' and issue state '{}'.",
        item.issue.number,
        item.issue.title,
        item.field_values
            .iter()
            .find(|field_value| option_ids.contains(&field_value.option_id))
            .unwrap()
            .name,
        &item.issue.state.to_string().to_lowercase()
    );

    // Change the issue state.
    let issue = client
        .issues(&item.issue.repository.owner.login, &item.issue.repository.name)
        .update(item.issue.number)
        .state(issue_state)
        .send()
        .await?;

    info!(
        "Issue #{} has now new issue state '{}'.",
        issue.number, &issue.state
    );

    Ok(())
}

/// Get the respective option ids for the given list of statuses.
async fn get_option_ids(status_field: Option<&Field>, statuses: &Vec<String>) -> Vec<String> {
    let mut option_ids: Vec<String> = Vec::new();
    for status_name in statuses.iter() {
        let option = status_field
            .unwrap()
            .options
            .iter()
            .find(|option| &option.name == status_name);

        match option {
            Some(option) => option_ids.push(option.id.clone()),
            None => {
                error!("Couldn't find status '{}'.", status_name);
                std::process::exit(1);
            }
        }
    }
    option_ids
}

async fn process_issue_batch(
    client: &Octocrab,
    args: &Args,
    cursor: &Option<String>,
) -> Result<PageInfo> {
    // Request the project and issue data.
    let body = json!({
        "query": include_str!("query.graphql"),
        "variables": {
            "owner": &args.owner,
            "project_number": args.project_number,
            "cursor": cursor,
        }
    });
    let response: Response = client.post("graphql", Some(&body)).await?;
    trace!("{response:#?}");

    // Inform the user if either the project or owner cannot be found.
    if let Some(owner) = &response.data.repository_owner {
        if owner.project.is_none() {
            error!(
                "Couldn't find project #{} for owner '{}'.",
                args.project_number, &args.owner
            );
            std::process::exit(1);
        }
    } else {
        error!("Couldn't find owner '{}'.", &args.owner);
        std::process::exit(1);
    }

    // Extract the simplified project from the response.
    let project: Project = response.into();
    if cursor.is_none() {
        info!(
            "Looking at project #{} '{}'.",
            args.project_number, project.title
        );
        trace!("{project:#?}");
    }

    // Extract the status field.
    let status_field = project.fields.iter().find(|field| field.name == "Status");
    if status_field.is_none() {
        error!("Something went wrong! There is no 'Status' field.");
        std::process::exit(1);
    }

    // We need the option ids of the closed and open statuses to check
    // if an item is in one of the target columns.
    let closed_option_ids = get_option_ids(status_field, &args.closed_statuses).await;
    let open_option_ids = get_option_ids(status_field, &args.open_statuses).await;

    info!("Found {} issues.", project.items.len());
    // Ensure the issue state for every item.
    for item in project.items {
        ensure_issue_state(
            &item,
            IssueState::Closed,
            &args,
            &client,
            &closed_option_ids,
        )
        .await?;

        ensure_issue_state(&item, IssueState::Open, &args, &client, &open_option_ids).await?;
    }

    Ok(project.page_info)
}
