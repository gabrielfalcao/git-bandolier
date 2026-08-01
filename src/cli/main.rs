use clap::{Parser, Subcommand};

use crate::cli::commands::{
    BranchesOpt,
    CommitDatedOpt,
    MangenOpt,
    // IgnoreOpt,
    PathOpt,
    QuickCommitOpt,
    RemotesOpt,
};
use crate::{
    Error,
    Exit,
    Result,
    dispatch::{ParserDispatcher, SubcommandDispatcher},
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "git_bandolier command-line")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Br(BranchesOpt),
    Branches(BranchesOpt),
    CommitDated(CommitDatedOpt),
    GenerateManPages(MangenOpt),
    // Ignore(IgnoreOpt),
    ListBranches(BranchesOpt),
    ListRemotes(RemotesOpt),
    Path(PathOpt),
    QuickCommit(QuickCommitOpt),
    Remotes(RemotesOpt),
}
impl SubcommandDispatcher<Error> for Command {
    fn dispatch(&self) -> Result<()> {
        match self {
            Command::Br(op) => op.dispatch()?,
            Command::Branches(op) => op.dispatch()?,
            Command::CommitDated(op) => op.dispatch()?,
            Command::GenerateManPages(op) => op.dispatch()?,
            // Command::Ignore(op) => op.dispatch()?,
            Command::ListBranches(op) => op.dispatch()?,
            Command::ListRemotes(op) => op.dispatch()?,
            Command::Path(op) => op.dispatch()?,
            Command::QuickCommit(op) => op.dispatch()?,
            Command::Remotes(op) => op.dispatch()?,
        }
        Ok(())
    }
}
