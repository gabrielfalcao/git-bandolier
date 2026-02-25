use clap::{Args, Subcommand};

use crate::cli::commands::export::{ExportDirOpt, ExportFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ExportCommand {
    Dir(ExportDirOpt),
    File(ExportFileOpt),
}
impl SubcommandDispatcher<Error> for ExportCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ExportCommand::Dir(op) => {
                op.dispatch()?;
            },
            ExportCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ExportOpt {
    #[command(subcommand)]
    command: ExportCommand,
}

impl ArgsDispatcher<Error> for ExportOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
