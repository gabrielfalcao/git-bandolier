use clap::{Args, Subcommand};

use crate::cli::commands::env::{EnvDirOpt, EnvFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum EnvCommand {
    Dir(EnvDirOpt),
    File(EnvFileOpt),
}
impl SubcommandDispatcher<Error> for EnvCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            EnvCommand::Dir(op) => {
                op.dispatch()?;
            },
            EnvCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct EnvOpt {
    #[command(subcommand)]
    command: EnvCommand,
}

impl ArgsDispatcher<Error> for EnvOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
