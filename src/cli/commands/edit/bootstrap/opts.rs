use clap::Args;
use iocore::Path;

use crate::cli::commands::bootstrap::shared::BootstrapSharedOpt;
use crate::dispatch::ArgsDispatcher;
use crate::{Error, Result};

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BootstrapDirOpt {
    #[clap(flatten)]
    opt: BootstrapSharedOpt,
}

impl BootstrapDirOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for BootstrapDirOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BootstrapFileOpt {
    #[clap(flatten)]
    opt: BootstrapSharedOpt,
}
impl BootstrapFileOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for BootstrapFileOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}
