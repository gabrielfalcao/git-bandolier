use clap::{Args, Subcommand};

use crate::cli::commands::save::{SaveDirOpt, SaveFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum SaveCommand {
    Dir(SaveDirOpt),
    File(SaveFileOpt),
}
impl SubcommandDispatcher<Error> for SaveCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            SaveCommand::Dir(op) => {
                op.dispatch()?;
            },
            SaveCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct SaveOpt {
    #[command(subcommand)]
    command: SaveCommand,
}

impl ArgsDispatcher<Error> for SaveOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
