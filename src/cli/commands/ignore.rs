use clap::Parser;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result};
use chrono::{DateTime, Utc};
use git2::Oid;
use git2::Repository;
use iocore::Path;

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IgnoreOpt {
    #[command(subcommand)]
    command: GitIgnoreCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum GitIgnoreCommand {
    Add(GitIgnoreAddOpt),
}
impl SubcommandDispatcher<Error> for GitIgnoreCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            GitIgnoreCommand::Add(op) => op.dispatch()?,
        }
        Ok(())
    }
}


impl IgnoreOpt {
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }
}

impl ParserDispatcher<Error> for IgnoreOpt {
    fn dispatch(&self) -> Result<()> {
        let git = self.git_repo()?;
        Ok(())
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
