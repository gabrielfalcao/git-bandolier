use clap::{Args, Subcommand};

use crate::cli::commands::show::{ShowDirOpt, ShowFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ShowCommand {
    Dir(ShowDirOpt),
    File(ShowFileOpt),
}
impl SubcommandDispatcher<Error> for ShowCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ShowCommand::Dir(op) => {
                op.dispatch()?;
            },
            ShowCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ShowOpt {
    #[command(subcommand)]
    command: ShowCommand,
}

impl ArgsDispatcher<Error> for ShowOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
