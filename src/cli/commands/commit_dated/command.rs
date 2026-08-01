use clap::{Parser, Subcommand};

use crate::cli::commands::commit_dated::{
    CommitDatedDirOpt, CommitDatedFileOpt,
};
use crate::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum CommitDatedCommand
{
    Dir(CommitDatedDirOpt),
    File(CommitDatedFileOpt),
}
impl SubcommandDispatcher<Error> for CommitDatedCommand
{
    fn dispatch(&self) -> Result<()>
    {
        match self
        {
            CommitDatedCommand::Dir(op) =>
            {
                op.dispatch()?;
            },
            CommitDatedCommand::File(op) =>
            {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct CommitDatedOpt
{
    #[command(subcommand)]
    command: CommitDatedCommand,
}

impl ParserDispatcher<Error> for CommitDatedOpt
{
    fn dispatch(&self) -> Result<()>
    {
        self.command.dispatch()?;
        Ok(())
    }
}
