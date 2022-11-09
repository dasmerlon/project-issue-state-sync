use anyhow::Result;
use clap::Parser;
use octocrab::models;
use serde_json::json;

mod args;
mod raw_response;
mod response;

use args::Args;
use raw_response::Response;
use response::Project;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let instance = octocrab::Octocrab::builder()
        .personal_token(args.github_token)
        .build()?;

    let body = json!({
        "query": include_str!("query.graphql"),
        "variables": {
            "owner": args.owner,
            "repository": args.repository,
            "project_number": args.project_number
        }
    });

    let response: Response = instance.post("graphql", Some(&body)).await?;
    let project: Project = response.into();

    let issue = instance
        .issues(args.owner, args.repository)
        .update(21)
        .state(args.issue_state)
        .send()
        .await?;

    dbg!(issue);
    Ok(())
}
