use clap::{Args, Subcommand};

use crate::cli::commands::refresh::{RefreshDirOpt, RefreshFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum RefreshCommand {
    Dir(RefreshDirOpt),
    File(RefreshFileOpt),
}
impl SubcommandDispatcher<Error> for RefreshCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            RefreshCommand::Dir(op) => {
                op.dispatch()?;
            },
            RefreshCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct RefreshOpt {
    #[command(subcommand)]
    command: RefreshCommand,
}

impl ArgsDispatcher<Error> for RefreshOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
