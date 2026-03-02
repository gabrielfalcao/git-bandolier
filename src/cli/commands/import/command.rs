use clap::{Args, Subcommand};

use crate::cli::commands::import::{ImportDirOpt, ImportFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ImportCommand {
    Dir(ImportDirOpt),
    File(ImportFileOpt),
}
impl SubcommandDispatcher<Error> for ImportCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ImportCommand::Dir(op) => {
                op.dispatch()?;
            },
            ImportCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ImportOpt {
    #[command(subcommand)]
    command: ImportCommand,
}

impl ArgsDispatcher<Error> for ImportOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
