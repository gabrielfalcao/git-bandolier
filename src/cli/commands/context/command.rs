use clap::{Args, Subcommand};

use crate::cli::commands::context::{ContextDirOpt, ContextFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ContextCommand {
    Dir(ContextDirOpt),
    File(ContextFileOpt),
}
impl SubcommandDispatcher<Error> for ContextCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ContextCommand::Dir(op) => {
                op.dispatch()?;
            },
            ContextCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ContextOpt {
    #[command(subcommand)]
    command: ContextCommand,
}

impl ArgsDispatcher<Error> for ContextOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
