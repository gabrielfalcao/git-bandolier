use clap::Parser;

use git2::{Repository, Status};

use crate::{Error, Result, dispatch::ParserDispatcher};
use iocore::Path;
use sanitation::SString;

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct QuickCommitListOpt {
    #[arg()]
    path: Option<Path>,
}
impl QuickCommitListOpt {
    pub fn path(&self) -> Path {
        self.path.clone().unwrap_or_else(|| Path::cwd())
    }
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(self.path().into())?)
    }
}

pub fn entry_status_to_string(entry: Status) -> &'static str {
    match entry {
        Status::CURRENT => "CURRENT",
        Status::INDEX_NEW => "INDEX_NEW",
        Status::INDEX_MODIFIED => "INDEX_MODIFIED",
        Status::INDEX_DELETED => "INDEX_DELETED",
        Status::INDEX_RENAMED => "INDEX_RENAMED",
        Status::INDEX_TYPECHANGE => "INDEX_TYPECHANGE",
        Status::WT_NEW => "WT_NEW",
        Status::WT_MODIFIED => "WT_MODIFIED",
        Status::WT_DELETED => "WT_DELETED",
        Status::WT_TYPECHANGE => "WT_TYPECHANGE",
        Status::WT_RENAMED => "WT_RENAMED",
        Status::WT_UNREADABLE => "WT_UNREADABLE",
        Status::IGNORED => "IGNORED",
        Status::CONFLICTED => "CONFLICTED",
        _ => unreachable!(),
    }
}
impl ParserDispatcher<Error> for QuickCommitListOpt {
    fn dispatch(&self) -> Result<()> {
        let repo = self.git_repo()?;
        let _state = repo.state();
        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(true);
        opts.exclude_submodules(true);

        let status = repo.statuses(Some(&mut opts))?;

        for entry in status.iter() {
            let path = SString::new(&entry.path_bytes()).safe()?;
            let status = entry_status_to_string(entry.status());
            println!("{path}\t{status}");
        }
        Ok(())
    }
}
