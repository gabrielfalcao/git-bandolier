use clap::{Args, Subcommand};

use crate::cli::commands::server::{ServerDirOpt, ServerFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ServerCommand {
    Dir(ServerDirOpt),
    File(ServerFileOpt),
}
impl SubcommandDispatcher<Error> for ServerCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ServerCommand::Dir(op) => {
                op.dispatch()?;
            },
            ServerCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ServerOpt {
    #[command(subcommand)]
    command: ServerCommand,
}

impl ArgsDispatcher<Error> for ServerOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
