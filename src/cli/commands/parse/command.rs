use clap::{Args, Subcommand};

use crate::cli::commands::parse::{ParseDirOpt, ParseFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum ParseCommand {
    Dir(ParseDirOpt),
    File(ParseFileOpt),
}
impl SubcommandDispatcher<Error> for ParseCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ParseCommand::Dir(op) => {
                op.dispatch()?;
            },
            ParseCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct ParseOpt {
    #[command(subcommand)]
    command: ParseCommand,
}

impl ArgsDispatcher<Error> for ParseOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
