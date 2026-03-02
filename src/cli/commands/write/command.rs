use clap::{Args, Subcommand};

use crate::cli::commands::write::{WriteDirOpt, WriteFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum WriteCommand {
    Dir(WriteDirOpt),
    File(WriteFileOpt),
}
impl SubcommandDispatcher<Error> for WriteCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            WriteCommand::Dir(op) => {
                op.dispatch()?;
            },
            WriteCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct WriteOpt {
    #[command(subcommand)]
    command: WriteCommand,
}

impl ArgsDispatcher<Error> for WriteOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
