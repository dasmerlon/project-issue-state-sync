use anyhow::Result;
use clap::Parser;
use log::{debug, error, info, trace};
use serde_json::json;

mod args;
mod raw_response;
mod response;

use args::Args;
use raw_response::Response;
use response::Project;
use simplelog::{Config, LevelFilter, SimpleLogger};

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

    let instance = octocrab::Octocrab::builder()
        .personal_token(args.github_token)
        .build()?;

    let body = json!({
        "query": include_str!("query.graphql"),
        "variables": {
            "owner": &args.owner,
            "project_number": args.project_number
        }
    });

    let response: Response = instance.post("graphql", Some(&body)).await?;

    // Inform the user if either the project or owner cannot be found
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

    let project: Project = response.into();
    info!("Looking at project {}.", project.title);
    trace!("{project:?}");

    for item in project.items {
        // Ignore items that aren't in the target column
        let has_correct_status = item
            .field_values
            .iter()
            .any(|field_value| field_value.name == args.status);

        if !has_correct_status {
            continue;
        }

        // Ignore issues that already have the desired state
        if args.issue_state == item.issue.state.clone().into() {
            continue;
        }

        info!(
            "Found issue #{} ({}) in column '{}' and issue state '{:?}'.",
            item.issue.number, item.issue.title, args.status, &item.issue.state
        );
        let issue = instance
            .issues(args.owner.clone(), item.issue.repository.name)
            .update(item.issue.number)
            .state(args.issue_state.clone())
            .send()
            .await?;

        info!(
            "Issue #{} has now new issue state '{:?}'.",
            issue.number, &issue.state
        );
    }

    info!("All done.");

    Ok(())
}
