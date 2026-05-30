use clap::Parser;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result};
use chrono::{DateTime, Utc};
use git2::Oid;
use git2::Repository;
use iocore::Path;
use sanitation::SString;

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BranchesOpt {}

impl BranchesOpt {
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NamedBranchInfo {
    pub name: String,
    pub commit_hash: Oid,
    pub datetime: DateTime<Utc>,
}
impl std::fmt::Display for NamedBranchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hash = self.commit_hash.to_string();
        let date = self.datetime.to_string();
        let name = self.name.to_string();
        write!(f, "{hash} {date} {name}")
    }
}
impl ParserDispatcher<Error> for BranchesOpt {
    fn dispatch(&self) -> Result<()> {
        let git = self.git_repo()?;
        let mut branches = git
            .branches(Some(git2::BranchType::Local))?
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .map(|(branch, ty)| {
                let name = branch.name().unwrap().map(|name| name.to_string()).unwrap();
                let commit = branch.into_reference().peel_to_commit().unwrap();
                let commit_hash = commit.id();
                let datetime = DateTime::from_timestamp(commit.time().seconds(), 0).unwrap();
                return NamedBranchInfo {
                    name,
                    commit_hash,
                    datetime,
                };
            })
            .collect::<Vec<NamedBranchInfo>>();

        branches.sort_by_key(|info| info.datetime);

        for br in branches {
            println!("{br}")
        }

        Ok(())
    }
}
