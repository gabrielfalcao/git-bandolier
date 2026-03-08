use clap::{Args, Subcommand};

use crate::cli::commands::task::{TaskDirOpt, TaskFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum TaskCommand {
    Dir(TaskDirOpt),
    File(TaskFileOpt),
}
impl SubcommandDispatcher<Error> for TaskCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            TaskCommand::Dir(op) => {
                op.dispatch()?;
            },
            TaskCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Args, Debug, Clone)]
pub struct TaskOpt {
    #[command(subcommand)]
    command: TaskCommand,
}

impl ArgsDispatcher<Error> for TaskOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
