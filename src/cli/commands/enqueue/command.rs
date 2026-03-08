use clap::{Args, Subcommand};

use crate::cli::commands::enqueue::{EnqueueDirOpt, EnqueueFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum EnqueueCommand {
    Dir(EnqueueDirOpt),
    File(EnqueueFileOpt),
}
impl SubcommandDispatcher<Error> for EnqueueCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            EnqueueCommand::Dir(op) => {
                op.dispatch()?;
            },
            EnqueueCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct EnqueueOpt {
    #[command(subcommand)]
    command: EnqueueCommand,
}

impl ArgsDispatcher<Error> for EnqueueOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
