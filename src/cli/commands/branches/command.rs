use clap::{Args, Subcommand, Parser};

use crate::cli::commands::branches::{BranchesDirOpt, BranchesFileOpt};
use crate::dispatch::{ArgsDispatcher, SubcommandDispatcher, ParserDispatcher};
use crate::{Error, Result};

#[derive(Subcommand, Debug, Clone)]
pub enum BranchesCommand {
    Dir(BranchesDirOpt),
    File(BranchesFileOpt),
}
impl SubcommandDispatcher<Error> for BranchesCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            BranchesCommand::Dir(op) => {
                op.dispatch()?;
            },
            BranchesCommand::File(op) => {
                op.dispatch()?;
            },
        }
        Ok(())
    }
}


#[derive(Parser, Debug, Clone)]
pub struct BranchesOpt {
    #[command(subcommand)]
    command: BranchesCommand,
}

impl ParserDispatcher<Error> for BranchesOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;
        Ok(())
    }
}
