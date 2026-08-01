use clap::Args;
use iocore::Path;

use crate::{Error, Result, cli::commands::switch::shared::SwitchSharedOpt, dispatch::ArgsDispatcher};

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SwitchDirOpt {
    #[clap(flatten)]
    opt: SwitchSharedOpt,
}

impl SwitchDirOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for SwitchDirOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SwitchFileOpt {
    #[clap(flatten)]
    opt: SwitchSharedOpt,
}
impl SwitchFileOpt {
    pub fn path(&self) -> Path {
        self.opt.path()
    }
}

impl ArgsDispatcher<Error> for SwitchFileOpt {
    fn dispatch(&self) -> Result<()> {
        let path = self.path();
        println!("path: {path}");
        Ok(())
    }
}
