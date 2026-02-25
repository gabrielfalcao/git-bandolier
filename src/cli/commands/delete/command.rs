use clap::{Args, Subcommand};

use crate::cli::commands::delete::{DeleteDirOpt, DeleteFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum DeleteCommand {
    Dir(DeleteDirOpt),
    File(DeleteFileOpt),
}
impl SubcommandDispatcher<Error> for DeleteCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            DeleteCommand::Dir(op) => {
                op.dispatch()?;
            },
            DeleteCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct DeleteOpt {
    #[command(subcommand)]
    command: DeleteCommand,
}

impl ArgsDispatcher<Error> for DeleteOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
