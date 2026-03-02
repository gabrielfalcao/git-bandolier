use clap::Args;
use iocore::Path;

use crate::cli::commands::init::shared::InitSharedOpt;
use crate::dispatch::ArgsDispatcher;
use crate::{Error, Result};

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InitDirOpt {
    #[clap(flatten)]
    opt: InitSharedOpt,
}

impl InitDirOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for InitDirOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InitFileOpt {
    #[clap(flatten)]
    opt: InitSharedOpt,
}
impl InitFileOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for InitFileOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}
