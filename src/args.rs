use clap::Parser;

use crate::response::IssueState;

#[derive(Parser, Debug)]
#[clap(
    name = "TODO",
    about = "TODO",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION")
)]
pub struct Args {
    /// The Github token for API calls.
    #[arg(short, long, env)]
    pub github_token: String,

    /// The User or Organization owner that owns the repository.
    #[arg(short, long, env)]
    pub owner: String,

    /// The Repository name.
    #[arg(short, long, env)]
    pub repository: String,

    /// The number of the project you target, for example, #1.
    #[arg(short, long, env)]
    pub project_number: usize,

    /// The project board column name.
    /// For example, "Todo", "In Progress" or "Done".
    #[arg(short, long, env)]
    pub status: String,

    /// The desired issue state for issues in the specified column.
    #[arg(short, long, env)]
    pub issue_state: IssueState,
}
