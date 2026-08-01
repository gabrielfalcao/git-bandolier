use clap::{Parser, Subcommand};

use crate::{
    Error,
    Result,
    cli::commands::switch::{SwitchDirOpt, SwitchFileOpt},
    dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher},
};

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
            }
            SwitchCommand::File(op) => {
                op.dispatch()?;
            }
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct SwitchOpt {
    #[command(subcommand)]
    command: SwitchCommand,
}

impl ParserDispatcher<Error> for SwitchOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
