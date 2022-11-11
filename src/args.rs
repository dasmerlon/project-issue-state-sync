use clap::{ArgAction, Parser};

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

    /// The number of the project you target, for example, #1.
    #[arg(short, long, env)]
    pub project_number: usize,

    /// The project board column names in which issues should be closed.
    /// For example: "Won't do","Done"
    /// (Make sure there are no spaces between arguments.)
    #[arg(short, long, env, value_delimiter(','))]
    pub closed_stati: Vec<String>,

    /// The project board column names in which issues should be open.
    /// For example: "Todo","In Progress"
    /// (Make sure there are no spaces between arguments.)
    #[arg(short('r'), long, env, value_delimiter(','))]
    pub open_stati: Vec<String>,

    /// Verbose mode (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}
