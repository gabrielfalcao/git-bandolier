use clap::Args;
use iocore::Path;

use crate::cli::commands::server::shared::ServerSharedOpt;
use crate::dispatch::ArgsDispatcher;
use crate::{Error, Result};

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServerDirOpt {
    #[clap(flatten)]
    opt: ServerSharedOpt,
}

impl ServerDirOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for ServerDirOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServerFileOpt {
    #[clap(flatten)]
    opt: ServerSharedOpt,
}
impl ServerFileOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for ServerFileOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}
