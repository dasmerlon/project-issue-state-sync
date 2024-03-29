use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "Project Issue State Sync",
    about = "Set issue states depending on their project columns.",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION")
)]
pub struct Args {
    /// The Github token for API calls.
    #[arg(short, long, env = "INPUT_GITHUB_TOKEN")]
    pub github_token: String,

    /// The User or Organization owner that owns the repository.
    #[arg(short, long, env = "INPUT_OWNER")]
    pub owner: String,

    /// The number of the project you target, for example, #1.
    #[arg(short, long, env = "INPUT_PROJECT_NUMBER")]
    pub project_number: usize,

    /// The project board column names in which issues should be closed.
    /// For example: "Won't do","Done"
    /// (Make sure there are no spaces between arguments.)
    #[arg(short, long, env = "INPUT_CLOSED_STATUSES", value_delimiter(','))]
    pub closed_statuses: Vec<String>,

    /// The project board column names in which issues should be open.
    /// For example: "Todo","In Progress"
    /// (Make sure there are no spaces between arguments.)
    #[arg(short('r'), long, env = "INPUT_OPEN_STATUSES", value_delimiter(','))]
    pub open_statuses: Vec<String>,

    /// Log output verbosity (info, debug, trace).
    #[arg(short, long, env = "INPUT_VERBOSITY", default_value = "info")]
    pub verbosity: LogLevel,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Info,
    Debug,
    Trace,
}
