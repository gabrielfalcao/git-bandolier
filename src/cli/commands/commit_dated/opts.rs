#![allow(unused)]
use clap::{Args, Parser};
use iocore::Path;

use crate::cli::commands::commit_dated::shared::CommitDatedSharedOpt;
use crate::dispatch::{ArgsDispatcher, ParserDispatcher};
use crate::{Error, Result};

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommitDatedDirOpt {
    #[clap(flatten)]
    opt: CommitDatedSharedOpt,
}

impl CommitDatedDirOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ParserDispatcher<Error> for CommitDatedDirOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommitDatedFileOpt {
    #[clap(flatten)]
    opt: CommitDatedSharedOpt,
}
impl CommitDatedFileOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for CommitDatedFileOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}
