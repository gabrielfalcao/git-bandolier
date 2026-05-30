use clap::Args;
use iocore::Path;

use crate::dispatch::ArgsDispatcher;
use crate::{Error, Result};
use git2::{Repository, Remote};

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
            .iter()
            .filter(|name|name.is_some())
            .map(|name|name.unwrap())
            .map(|name|git.find_remote(name))
            .filter(|name|name.is_ok())
            .map(|name|name.unwrap())
            .map(|remote|(remote.name().map(|name|name.to_string()), remote.pushurl().map(|url|url.to_string())))
            .filter(|(name, url)|name.is_some())
            .map(|(name, url)|(name.unwrap().to_string(), url))
            .collect::<Vec<(String, Option<String>)>>();
        for (name, url) in remotes {
            match url {
                Some(url) => {
                    println!("{name}\t{url}");
                },
                None => {
                    println!("{name}\t<no url>");
                },

            }
        }
        Ok(())
    }
}
