use clap::{Args, Subcommand};

use crate::cli::commands::client::{ClientDirOpt, ClientFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ClientCommand {
    Dir(ClientDirOpt),
    File(ClientFileOpt),
}
impl SubcommandDispatcher<Error> for ClientCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ClientCommand::Dir(op) => {
                op.dispatch()?;
            },
            ClientCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ClientOpt {
    #[command(subcommand)]
    command: ClientCommand,
}

impl ArgsDispatcher<Error> for ClientOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
