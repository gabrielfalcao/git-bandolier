use clap::Parser;
use iocore::Path;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result};
use git2::Repository;

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RemotesOpt {}

impl RemotesOpt {
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }
}

impl ParserDispatcher<Error> for RemotesOpt {
    fn dispatch(&self) -> Result<()> {
        let git = self.git_repo()?;
        let remotes = git
            .remotes()?
            .iter()
            .filter(|name| name.is_some())
            .map(|name| name.unwrap())
            .map(|name| git.find_remote(name))
            .filter(|remote| remote.is_ok())
            .map(|name| name.unwrap())
            .map(|remote| {
                (
                    remote.name().map(|name| name.to_string()),
                    remote.pushurl().map(|url| url.to_string()).or(remote.url().map(|url|url.to_string())),
                )
            })
            .filter(|(name, _url)| name.is_some())
            .map(|(name, url)| {
                (
                    name.unwrap().to_string(),
                    url.unwrap_or_else(|| "<no-url>".to_string()).to_string(),
                )
            })
            .collect::<Vec<(String, String)>>();
        for (name, url) in remotes {
            println!("{name} => {url}");
        }
        Ok(())
    }
}
