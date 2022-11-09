use anyhow::Result;
use serde_json::json;

mod raw_response;
mod response;

use raw_response::Response;
use response::Project;

#[tokio::main]
async fn main() -> Result<()> {
    let instance = octocrab::Octocrab::builder()
        .personal_token("ADD TOKEN HERE")
        .build()?;

    let body = json!({
        "query": include_str!("query.graphql"),
        "variables": {
            "owner": "Tiefgang",
            "repo": "Orga",
            "project": 1
        }
    });

    let response: Response = instance.post("graphql", Some(&body)).await?;
    let project: Project = response.into();
    dbg!(project);
    Ok(())
}
