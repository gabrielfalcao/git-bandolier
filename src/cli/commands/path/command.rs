use clap::Parser;
use iocore::Path;

use crate::dispatch::ParserDispatcher;
use crate::{discover_git_repo, Error, Result};

#[derive(Parser, Debug, Clone)]
pub struct PathOpt {
    #[arg(
        help = "path to be used as starting point to discover the git repo path, defaults to the current working directory"
    )]
    starting_point: Option<Path>,
}
impl PathOpt {
    pub fn starting_point(&self) -> Path {
        self.starting_point.clone().unwrap_or_else(|| Path::cwd())
    }
}
impl ParserDispatcher<Error> for PathOpt {
    fn dispatch(&self) -> Result<()> {
        let starting_point = self.starting_point();
        match discover_git_repo(&starting_point) {
            Ok((repo, path)) => {
                if repo.is_bare() {
                    println!("{path}");
                } else {
                    let repr = format!("{:#?}", path.to_string());
                    let parent = path.parent().expect(&format!("parent path to {repr}"));
                    println!("{parent}");
                }
                // println!("branch: {branch_name}");
            }
            Err(error) => {
                let path = starting_point.to_string();
                eprintln!("path {path:#?} is not versioned by git: {error}");
                std::process::exit(3);
            }
        }
        Ok(())
    }
}
