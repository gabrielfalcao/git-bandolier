use clap::Parser;
use git2::Repository;
use iocore::Path;

use crate::cli::commands::switch::{PathDirOpt, PathFileOpt};
use crate::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

pub fn discover_git_repo(&self, starting_point: &Path) -> Result<Repository> {
    Ok(Repository::discover::<Path>(starting_point.into())?)
}

#[derive(Parser, Debug, Clone)]
pub struct PathOpt {
    #[arg(
        help = "path to be used as starting point to discover the git repo path, defaults to the current working directory"
    )]
    starting_point: Option<Path>,
}
impl PathOpt {
    pub fn starting_point(&self) -> Path {
        self.starting_point
            .clone()
            .unwrap_or_else(|| Path::cwd())
    }
}
impl ParserDispatcher<Error> for PathOpt {
    fn dispatch(&self) -> Result<()> {
        let starting_point = self.starting_point();
        match discover_git_repo(&starting_point) {
            Ok(repo) => {
                println!("{repo}");
            },
            Err(error) => {
                let path = starting_point.to_string();
                eprintln!("path {:#?} is not versioned by git: {error}");
                std::process::exit(404 % u8::MAX.into());
            },
        }
        Ok(())
    }
}
