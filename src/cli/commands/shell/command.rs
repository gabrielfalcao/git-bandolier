use clap::{Args, Subcommand};

use crate::cli::commands::shell::{ShellDirOpt, ShellFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ShellCommand {
    Dir(ShellDirOpt),
    File(ShellFileOpt),
}
impl SubcommandDispatcher<Error> for ShellCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ShellCommand::Dir(op) => {
                op.dispatch()?;
            },
            ShellCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ShellOpt {
    #[command(subcommand)]
    command: ShellCommand,
}

impl ArgsDispatcher<Error> for ShellOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
