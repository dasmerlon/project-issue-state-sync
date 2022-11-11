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
use response::{Field, Item, Project};
use simplelog::{Config, LevelFilter, SimpleLogger};

use crate::response::IssueState;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Set the verbosity level of the logger.
    let level = match args.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Trace,
    };
    SimpleLogger::init(level, Config::default()).unwrap();

    // Inform the user if neither closed_stati nor open_stati is specified.
    if args.closed_stati.is_empty() && args.open_stati.is_empty() {
        error!("No project board column names were specified.");
        std::process::exit(1);
    }

    // Initialize the Github API client.
    let instance = octocrab::Octocrab::builder()
        .personal_token(args.github_token.clone())
        .build()?;

    // Request the project and issue data.
    let body = json!({
        "query": include_str!("query.graphql"),
        "variables": {
            "owner": &args.owner,
            "project_number": args.project_number
        }
    });
    let response: Response = instance.post("graphql", Some(&body)).await?;

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
    info!("Looking at project {}.", project.title);
    trace!("{project:?}");

    // Extract the status field.
    let status_field = project.fields.iter().find(|field| field.name == "Status");
    if status_field.is_none() {
        error!("Something went wrong! There is no 'Status' field.");
        std::process::exit(1);
    }

    // We need the option ids of the closed and open stati to check
    // if an item is in one of the target columns.
    let closed_option_ids = get_option_ids(status_field, &args.closed_stati).await;
    let open_option_ids = get_option_ids(status_field, &args.open_stati).await;

    // Ensure the issue state for every item.
    for item in project.items {
        ensure_issue_state(
            &item,
            IssueState::Closed,
            &args,
            &instance,
            &closed_option_ids,
        )
        .await?;

        ensure_issue_state(&item, IssueState::Open, &args, &instance, &open_option_ids).await?;
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
    instance: &Octocrab,
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
    if issue_state == item.issue.state.clone().into() {
        return Ok(());
    }

    info!(
        "Found issue #{} ({}) in column '{}' and issue state '{}'.",
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
    let issue = instance
        .issues(args.owner.clone(), &item.issue.repository.name)
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

/// Get the respective option ids for the given list of stati.
async fn get_option_ids(status_field: Option<&Field>, stati: &Vec<String>) -> Vec<String> {
    let mut option_ids: Vec<String> = Vec::new();
    for status_name in stati.iter() {
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
