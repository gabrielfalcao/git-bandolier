use clap::{Args, Subcommand};

use crate::cli::commands::today::{TodayDirOpt, TodayFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum TodayCommand {
    Dir(TodayDirOpt),
    File(TodayFileOpt),
}
impl SubcommandDispatcher<Error> for TodayCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            TodayCommand::Dir(op) => {
                op.dispatch()?;
            },
            TodayCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct TodayOpt {
    #[command(subcommand)]
    command: TodayCommand,
}

impl ArgsDispatcher<Error> for TodayOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
