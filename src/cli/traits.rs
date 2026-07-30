use clap::FromArgMatches;
use git2::{Repository, Status, StatusEntry, StatusOptions};
use iocore::Path;
use sanitation::SString;

use crate::{Error, Result};

pub trait GitRepoAutoDiscover: FromArgMatches {
    fn starting_point(&self) -> Path;
    fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(self.starting_point().into())?)
    }
    fn repo_path_from_repo(&self, repo: &Repository) -> Result<Path> {
        let git_bare_path = Path::from(repo.path());
        match git_bare_path.parent() {
            Some(path) => Ok(path),
            None => Err(Error::IOError(format!("cannot obtain parent of path {git_bare_path}"))),
        }
    }
    fn repo_path(&self) -> Result<Path> {
        let repo = self.git_repo()?;
        Ok(self.repo_path_from_repo(&repo)?)
    }
    fn git_ignore_path(&self) -> Result<Path> {
        let repo_path = self.repo_path()?;
        let ignore_path = repo_path.join(".gitignore");
        if ignore_path.exists() && !ignore_path.is_file() {
            return Err(Error::IOError(format!("gitignore exists but is not a file: {ignore_path}")));
        }
        Ok(ignore_path)
    }
    fn git_status_list_untracked_paths(&self, recurse_untracked: Option<bool>) -> Result<Vec<Path>> {
        let repo = self.git_repo()?;
        let repo_path = self.repo_path_from_repo(&repo)?;
        let mut options = StatusOptions::new();
        let mut options = options
            .include_untracked(true)
            .include_unmodified(false)
            .include_unreadable(false)
            .include_ignored(false)
            .exclude_submodules(true)
            .sort_case_sensitively(true)
            .recurse_untracked_dirs(recurse_untracked.unwrap_or_default());
        let statuses = repo.statuses(Some(&mut options))?;
        let mut result = Vec::<Path>::new();
        for item in statuses.iter() {
            let path_string = SString::new(item.path_bytes()).safe().unwrap_or_else(|error| {
                let fallback = hex::encode(item.path_bytes());
                log::warn!(
                    "failed to read UTF-8 string from git status entry: \
                         {error}. Falling back to hex representation: \
                         {fallback}"
                );
                fallback
            });
            let status_path = Path::new(path_string.as_str());
            let path = if status_path.is_absolute() {
                status_path
            } else {
                repo_path.join(path_string.as_str())
            };
            result.push(path);
        }
        Ok(result)
    }
}
