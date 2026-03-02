use clap::{Args, Subcommand};

use crate::cli::commands::update::{UpdateDirOpt, UpdateFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum UpdateCommand {
    Dir(UpdateDirOpt),
    File(UpdateFileOpt),
}
impl SubcommandDispatcher<Error> for UpdateCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            UpdateCommand::Dir(op) => {
                op.dispatch()?;
            },
            UpdateCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct UpdateOpt {
    #[command(subcommand)]
    command: UpdateCommand,
}

impl ArgsDispatcher<Error> for UpdateOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
