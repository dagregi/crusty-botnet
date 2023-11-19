use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
/// Botnet command center
pub struct Arguments {
    #[clap(subcommand)]
    pub sub_commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Executes the specified command on the hosts
    Execute(ExecuteCommand),
    /// Show the number of connected hosts
    Connections,
}

#[derive(Debug, Args)]
pub struct ExecuteCommand {
    pub commands: Vec<String>,
}
