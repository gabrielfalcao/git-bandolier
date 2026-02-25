use clap::{Args, Subcommand};

use crate::cli::commands::switch::{SwitchDirOpt, SwitchFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum SwitchCommand {
    Dir(SwitchDirOpt),
    File(SwitchFileOpt),
}
impl SubcommandDispatcher<Error> for SwitchCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            SwitchCommand::Dir(op) => {
                op.dispatch()?;
            },
            SwitchCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct SwitchOpt {
    #[command(subcommand)]
    command: SwitchCommand,
}

impl ArgsDispatcher<Error> for SwitchOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
