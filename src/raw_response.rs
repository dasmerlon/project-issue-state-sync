use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};

use crate::response::{FieldOption, Issue};

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: ResponseData,
}

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub repository_owner: Owner,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub project: Project,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub title: String,
    pub fields: Fields,
    pub items: Items,
}

#[derive(Deserialize, Debug)]
pub struct Fields {
    pub nodes: Vec<Field>,
}

#[derive(Deserialize, Debug)]
pub struct Items {
    pub nodes: Vec<Item>,
}

#[derive(Deserialize, Debug)]
pub struct Field {
    pub name: Option<String>,
    pub options: Option<Vec<FieldOption>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemType {
    Issue,
    DraftIssue,
    PullRequest,
    Redacted,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Item {
    pub field_values: FieldValues,
    pub id: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub issue: Option<Issue>,
}

#[derive(Deserialize, Debug)]
pub struct FieldValues {
    pub nodes: Vec<FieldValue>,
}

#[derive(Deserialize, Debug)]
pub struct FieldValue {
    pub name: Option<String>,
    pub option_id: Option<String>,
}
