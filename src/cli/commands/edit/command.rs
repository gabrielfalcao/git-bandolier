use clap::{Args, Subcommand};

use crate::cli::commands::edit::{EditDirOpt, EditFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum EditCommand {
    Dir(EditDirOpt),
    File(EditFileOpt),
}
impl SubcommandDispatcher<Error> for EditCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            EditCommand::Dir(op) => {
                op.dispatch()?;
            },
            EditCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct EditOpt {
    #[command(subcommand)]
    command: EditCommand,
}

impl ArgsDispatcher<Error> for EditOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
