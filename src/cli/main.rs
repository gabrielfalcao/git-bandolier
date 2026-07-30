use clap::{Parser, Subcommand};
use crate::cli::commands::{
    BranchesOpt,
    CommitDatedOpt,
    PathOpt,
    QuickCommitOpt,
    RemotesOpt,
    // IgnoreOpt,
};
use crate::dispatch::{ParserDispatcher, SubcommandDispatcher};
use crate::{Error, Exit, Result};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "git_bandolier command-line")]
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
        self.command().dispatch()?;
        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Path(PathOpt),
    CommitDated(CommitDatedOpt),
    Branches(BranchesOpt),
    // Ignore(IgnoreOpt),
    ListBranches(BranchesOpt),
    Br(BranchesOpt),
    Remotes(RemotesOpt),
    ListRemotes(RemotesOpt),
    QuickCommit(QuickCommitOpt),
}
impl SubcommandDispatcher<Error> for Command {
    fn dispatch(&self) -> Result<()> {
        match self {
            Command::CommitDated(op) => op.dispatch()?,
            Command::Path(op) => op.dispatch()?,
            Command::Remotes(op) => op.dispatch()?,
            Command::ListRemotes(op) => op.dispatch()?,
            Command::Br(op) => op.dispatch()?,
            Command::Branches(op) => op.dispatch()?,
            // Command::Ignore(op) => op.dispatch()?,
            Command::ListBranches(op) => op.dispatch()?,
            Command::QuickCommit(op) => op.dispatch()?,
        }
        Ok(())
    }
}
