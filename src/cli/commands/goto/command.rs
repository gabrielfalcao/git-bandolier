use clap::{Args, Subcommand};

use crate::cli::commands::goto::{GotoDirOpt, GotoFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum GotoCommand {
    Dir(GotoDirOpt),
    File(GotoFileOpt),
}
impl SubcommandDispatcher<Error> for GotoCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            GotoCommand::Dir(op) => {
                op.dispatch()?;
            },
            GotoCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct GotoOpt {
    #[command(subcommand)]
    command: GotoCommand,
}

impl ArgsDispatcher<Error> for GotoOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
