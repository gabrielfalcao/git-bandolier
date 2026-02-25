use clap::Args;
use iocore::Path;

use crate::cli::commands::web::shared::WebSharedOpt;
use crate::dispatch::ArgsDispatcher;
use crate::{Error, Result};

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WebDirOpt {
    #[clap(flatten)]
    opt: WebSharedOpt,
}

impl WebDirOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for WebDirOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WebFileOpt {
    #[clap(flatten)]
    opt: WebSharedOpt,
}
impl WebFileOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for WebFileOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}
