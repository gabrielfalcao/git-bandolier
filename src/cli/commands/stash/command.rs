use clap::{Args, Subcommand};

use crate::cli::commands::stash::{StashDirOpt, StashFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum StashCommand {
    Dir(StashDirOpt),
    File(StashFileOpt),
}
impl SubcommandDispatcher<Error> for StashCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            StashCommand::Dir(op) => {
                op.dispatch()?;
            },
            StashCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct StashOpt {
    #[command(subcommand)]
    command: StashCommand,
}

impl ArgsDispatcher<Error> for StashOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
