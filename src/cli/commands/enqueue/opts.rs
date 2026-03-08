use clap::Args;
use iocore::Path;

use crate::cli::commands::enqueue::shared::EnqueueSharedOpt;
use crate::dispatch::ArgsDispatcher;
use crate::{Error, Result};

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnqueueDirOpt {
    #[clap(flatten)]
    opt: EnqueueSharedOpt,
}

impl EnqueueDirOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for EnqueueDirOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnqueueFileOpt {
    #[clap(flatten)]
    opt: EnqueueSharedOpt,
}
impl EnqueueFileOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for EnqueueFileOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}
