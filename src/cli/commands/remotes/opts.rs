use clap::Args;
use iocore::Path;

use crate::cli::commands::remotes::shared::RemotesSharedOpt;
use crate::dispatch::ArgsDispatcher;
use crate::{Error, Result};

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RemotesDirOpt {
    #[clap(flatten)]
    opt: RemotesSharedOpt,
}

impl RemotesDirOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for RemotesDirOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RemotesFileOpt {
    #[clap(flatten)]
    opt: RemotesSharedOpt,
}
impl RemotesFileOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for RemotesFileOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}
