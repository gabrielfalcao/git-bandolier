use clap::{Args, Subcommand};

use crate::cli::commands::bootstrap::{BootstrapDirOpt, BootstrapFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum BootstrapCommand {
    Dir(BootstrapDirOpt),
    File(BootstrapFileOpt),
}
impl SubcommandDispatcher<Error> for BootstrapCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            BootstrapCommand::Dir(op) => {
                op.dispatch()?;
            },
            BootstrapCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct BootstrapOpt {
    #[command(subcommand)]
    command: BootstrapCommand,
}

impl ArgsDispatcher<Error> for BootstrapOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
