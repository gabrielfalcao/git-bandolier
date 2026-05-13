use clap::{Parser, Subcommand};

use crate::cli::commands::quick_commit::QuickCommitListOpt;
use crate::dispatch::{ParserDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum QuickCommitCommand {
    List(QuickCommitListOpt),
}
impl Default for QuickCommitCommand {
    fn default() -> QuickCommitCommand {
        QuickCommitCommand::List(Default::default())
    }
}
impl SubcommandDispatcher<Error> for QuickCommitCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            QuickCommitCommand::List(op) => {
                op.dispatch()?;
            }
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct QuickCommitOpt {
    #[command(subcommand)]
    command: QuickCommitCommand,
}

impl ParserDispatcher<Error> for QuickCommitOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
