use octocrab::models::IssueState as OctoIssueState;
use serde::Deserialize;

use crate::raw_response::{FieldValues, Response};

#[derive(Deserialize, clap::ValueEnum, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssueState {
    Open,
    Closed,
}

impl From<IssueState> for OctoIssueState {
    fn from(issue_state: IssueState) -> Self {
        match issue_state {
            IssueState::Open => OctoIssueState::Open,
            IssueState::Closed => OctoIssueState::Closed,
        }
    }
}

impl From<OctoIssueState> for IssueState {
    fn from(issue_state: OctoIssueState) -> Self {
        match issue_state {
            OctoIssueState::Open => IssueState::Open,
            OctoIssueState::Closed => IssueState::Closed,
            _ => IssueState::Open,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub id: String,
    pub number: u64,
    pub title: String,
    pub state: IssueState,
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct FieldValue {
    pub name: String,
    pub option_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub field_values: Vec<FieldValue>,
    pub id: String,
    pub issue: Issue,
}

#[derive(Deserialize, Debug)]
pub struct FieldOption {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub options: Vec<FieldOption>,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub title: String,
    pub fields: Vec<Field>,
    pub items: Vec<Item>,
}

impl From<Response> for Project {
    fn from(response: Response) -> Self {
        let project = response.data.repository_owner.project;

        let fields = project
            .fields
            .nodes
            .into_iter()
            .filter(|field| field.options.is_some() && field.name.is_some())
            .map(|field| Field {
                name: field.name.unwrap(),
                options: field.options.unwrap(),
            })
            .collect();

        let items = project
            .items
            .nodes
            .into_iter()
            .filter(|item| item.issue.is_some())
            .map(|item| Item {
                id: item.id,
                issue: item.issue.unwrap(),
                field_values: item.field_values.into(),
            })
            .collect();

        Project {
            title: project.title,
            fields,
            items,
        }
    }
}

impl From<FieldValues> for Vec<FieldValue> {
    fn from(field_values: FieldValues) -> Self {
        field_values
            .nodes
            .into_iter()
            .filter(|field_value| field_value.name.is_some() && field_value.option_id.is_some())
            .map(|field_value| FieldValue {
                name: field_value.name.unwrap(),
                option_id: field_value.option_id.unwrap(),
            })
            .collect()
    }
}
