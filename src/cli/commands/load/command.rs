use clap::{Args, Subcommand};

use crate::cli::commands::load::{LoadDirOpt, LoadFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum LoadCommand {
    Dir(LoadDirOpt),
    File(LoadFileOpt),
}
impl SubcommandDispatcher<Error> for LoadCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            LoadCommand::Dir(op) => {
                op.dispatch()?;
            },
            LoadCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct LoadOpt {
    #[command(subcommand)]
    command: LoadCommand,
}

impl ArgsDispatcher<Error> for LoadOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
