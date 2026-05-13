use clap::Parser;
use git2::Repository;
use iocore::Path;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result};

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct RemotesListOpt {
    #[arg()]
    path: Option<Path>,
}
impl RemotesListOpt {
    pub fn path(&self) -> Path {
        self.path.clone().unwrap_or_else(|| Path::cwd())
    }
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(self.path().into())?)
    }
}

impl ParserDispatcher<Error> for RemotesListOpt {
    fn dispatch(&self) -> Result<()> {
        let repo = self.git_repo()?;
        let quick-commit = repo.quick-commit()?;
        let total = quick-commit.len();
        for (index, op_name) in quick-commit.iter().enumerate() {
            let current = index + 1;
            match op_name.map(|name| repo.find_remote(name)) {
                Some(Ok(remote)) => {
                    let name = remote.name().map(|name|name.to_string()).unwrap();
                    let url = remote.url().map(|url|url.to_string()).unwrap();
                    println!("{name} {url}");
                }
                Some(Err(error)) => {
                    eprintln!("error retrieving remote {current} of {total}: {error}");
                }
                None => {
                    eprintln!("cannot find remote {current} of {total}");
                }
            }
        }
        Ok(())
    }
}
