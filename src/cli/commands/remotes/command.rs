use clap::{Parser, Subcommand};

use crate::cli::commands::remotes::{RemotesFileOpt, RemotesOpt};
use crate::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum RemotesCommand {
    Dir(RemotesOpt),
    File(RemotesFileOpt),
}
impl SubcommandDispatcher<Error> for RemotesCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            RemotesCommand::Dir(op) => {
                op.dispatch()?;
            }
            RemotesCommand::File(op) => {
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
