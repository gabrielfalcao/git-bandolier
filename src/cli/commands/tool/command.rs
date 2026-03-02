use clap::{Args, Subcommand};

use crate::cli::commands::tool::{ToolDirOpt, ToolFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ToolCommand {
    Dir(ToolDirOpt),
    File(ToolFileOpt),
}
impl SubcommandDispatcher<Error> for ToolCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ToolCommand::Dir(op) => {
                op.dispatch()?;
            },
            ToolCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ToolOpt {
    #[command(subcommand)]
    command: ToolCommand,
}

impl ArgsDispatcher<Error> for ToolOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
