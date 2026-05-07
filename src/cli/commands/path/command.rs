use clap::Parser;
use git2::Repository;
use iocore::Path;

use crate::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use crate::{Error, Result};

pub fn discover_git_repo(starting_point: &Path) -> Result<Repository> {
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
        self.starting_point.clone().unwrap_or_else(|| Path::cwd())
    }
}
impl ParserDispatcher<Error> for PathOpt {
    fn dispatch(&self) -> Result<()> {
        let starting_point = self.starting_point();
        match discover_git_repo(&starting_point) {
            Ok(repo) => {
                // let branch_name = repo
                //     .branches(Some(git2::BranchType::Local))?
                //     .map(|result| {
                //         result.map(|(branch, _branch_type)| {
                //             let branch_name = branch
                //                 .name()
                //                 .map(|oname| oname.map(|name| name.to_string())).unwrap();
                //             branch_name
                //         })
                //     })
                //     .reduce(|acc, e| {
                //         dbg!(&acc, &e);
                //         format!("{e:#?}")
                //         // e.unwrap().to_string()
                //     });
                let head = repo.head()?;
                let head_name = head
                    .name()
                    .map(|name| name.to_string())
                    .or_else(|| head.target().map(|target| hex::encode(target.as_bytes())))
                    .unwrap();

                println!("head: {head_name}");
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
