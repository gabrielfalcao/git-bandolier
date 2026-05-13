use clap::{Parser, Subcommand};

use crate::cli::commands::quick-commit::RemotesListOpt;
use crate::dispatch::{ParserDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum RemotesCommand {
    List(RemotesListOpt),
}
impl Default for RemotesCommand {
    fn default() -> RemotesCommand {
        RemotesCommand::List(Default::default())
    }
}
impl SubcommandDispatcher<Error> for RemotesCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            RemotesCommand::List(op) => {
                op.dispatch()?;
            }
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct RemotesOpt {
    #[command(subcommand)]
    command: RemotesCommand,
}

impl ParserDispatcher<Error> for RemotesOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
