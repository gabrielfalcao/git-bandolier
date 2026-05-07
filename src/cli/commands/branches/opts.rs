use clap::Parser;
use git2::Repository;
use iocore::Path;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result};

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BranchesOpt {}

impl BranchesOpt {
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }
}

impl ParserDispatcher<Error> for BranchesOpt {
    fn dispatch(&self) -> Result<()> {
        let git = self.git_repo()?;
        let branches = git.branches(Some(git2::BranchType::Local))?;
        for br in branches {
            match br {
                Ok((branch, ty)) => match branch.name() {
                    Ok(Some(name)) => {
                        println!("{name} {ty:#?}");
                    },
                    Ok(None) => {
                        eprintln!("could not get {ty:#?} branch name");
                    },
                    Err(error) => {
                        eprintln!("Error reading {ty:#?} branch name: {error}");
                    },
                },
                Err(error) => {
                    eprintln!("Error reading branch info: {error}");
                },
            }
        }
        Ok(())
    }
}
