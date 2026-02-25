use clap::{Args, Subcommand};

use crate::cli::commands::list::{ListDirOpt, ListFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ListCommand {
    Dir(ListDirOpt),
    File(ListFileOpt),
}
impl SubcommandDispatcher<Error> for ListCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ListCommand::Dir(op) => {
                op.dispatch()?;
            },
            ListCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ListOpt {
    #[command(subcommand)]
    command: ListCommand,
}

impl ArgsDispatcher<Error> for ListOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
