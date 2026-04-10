use clap::{Parser, Subcommand};
use git_bandolier::cli::commands::{SwitchOpt, WebOpt, CommitDatedOpt};
use git_bandolier::dispatch::{
    ArgsDispatcher, ParserDispatcher, SubcommandDispatcher,
};
use git_bandolier::{Error, Exit, Result};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "git_bandolier command-line"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let cmd = &self.command;
        dbg!(&cmd);

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Switch(SwitchOpt),
    CommitDated(CommitDatedOpt),
    Web(WebOpt),
}
impl SubcommandDispatcher<Error> for Command {
    fn dispatch(&self) -> Result<()> {
        match self {
            Command::CommitDated(op) => op.dispatch()?,
            Command::Switch(op) => op.dispatch()?,
            Command::Web(op) => op.dispatch()?,
        }
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
