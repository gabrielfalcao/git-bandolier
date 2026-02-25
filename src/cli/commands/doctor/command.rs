use clap::{Args, Subcommand};

use crate::cli::commands::doctor::{DoctorDirOpt, DoctorFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum DoctorCommand {
    Dir(DoctorDirOpt),
    File(DoctorFileOpt),
}
impl SubcommandDispatcher<Error> for DoctorCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            DoctorCommand::Dir(op) => {
                op.dispatch()?;
            },
            DoctorCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct DoctorOpt {
    #[command(subcommand)]
    command: DoctorCommand,
}

impl ArgsDispatcher<Error> for DoctorOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
