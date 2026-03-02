use clap::{Args, Subcommand};

use crate::cli::commands::find::{FindDirOpt, FindFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum FindCommand {
    Dir(FindDirOpt),
    File(FindFileOpt),
}
impl SubcommandDispatcher<Error> for FindCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            FindCommand::Dir(op) => {
                op.dispatch()?;
            },
            FindCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct FindOpt {
    #[command(subcommand)]
    command: FindCommand,
}

impl ArgsDispatcher<Error> for FindOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
