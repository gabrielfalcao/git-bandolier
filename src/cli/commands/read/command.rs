use clap::{Args, Subcommand};

use crate::cli::commands::read::{ReadDirOpt, ReadFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ReadCommand {
    Dir(ReadDirOpt),
    File(ReadFileOpt),
}
impl SubcommandDispatcher<Error> for ReadCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ReadCommand::Dir(op) => {
                op.dispatch()?;
            },
            ReadCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ReadOpt {
    #[command(subcommand)]
    command: ReadCommand,
}

impl ArgsDispatcher<Error> for ReadOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
