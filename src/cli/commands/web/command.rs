use clap::{Args, Subcommand};

use crate::cli::commands::web::{WebDirOpt, WebFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum WebCommand {
    Dir(WebDirOpt),
    File(WebFileOpt),
}
impl SubcommandDispatcher<Error> for WebCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            WebCommand::Dir(op) => {
                op.dispatch()?;
            },
            WebCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct WebOpt {
    #[command(subcommand)]
    command: WebCommand,
}

impl ArgsDispatcher<Error> for WebOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
