use clap::{Args, Subcommand};

use crate::cli::commands::init::{InitDirOpt, InitFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum InitCommand {
    Dir(InitDirOpt),
    File(InitFileOpt),
}
impl SubcommandDispatcher<Error> for InitCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            InitCommand::Dir(op) => {
                op.dispatch()?;
            },
            InitCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct InitOpt {
    #[command(subcommand)]
    command: InitCommand,
}

impl ArgsDispatcher<Error> for InitOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
