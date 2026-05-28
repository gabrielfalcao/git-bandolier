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
            let commit = br.into_reference().peel_to_commit()?;
            let header = SString::new(commit.raw_header_bytes()).safe()?;
            let name = branch.name()?;

        }
        Ok(())
    }
}
