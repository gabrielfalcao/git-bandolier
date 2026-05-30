use clap::Args;
use iocore::Path;

use crate::cli::commands::remotes::shared::RemotesSharedOpt;
use crate::dispatch::ArgsDispatcher;
use crate::{Error, Result};

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RemotesOpt {}

impl RemotesOpt {
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }
}

impl ArgsDispatcher<Error> for RemotesOpt {
    fn dispatch(&self) -> Result<()> {
        let git = self.git_repo()?;
        let remotes = git
            .remotes()?
            .map(|name| name.to_string())
            .collect::<Vec<String>>();
        for name in remotes {
            println!("{name}");
        }
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
