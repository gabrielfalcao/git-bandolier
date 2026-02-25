use clap::{Args, Subcommand};

use crate::cli::commands::path::{PathDirOpt, PathFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum PathCommand {
    Dir(PathDirOpt),
    File(PathFileOpt),
}
impl SubcommandDispatcher<Error> for PathCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            PathCommand::Dir(op) => {
                op.dispatch()?;
            },
            PathCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct PathOpt {
    #[command(subcommand)]
    command: PathCommand,
}

impl ArgsDispatcher<Error> for PathOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
