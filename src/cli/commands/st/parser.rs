#![allow(unused)]
use clap::Parser;
use couleur_rs::{Color, Contrast, Layer};
use git2::Repository;
use iocore::Path;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result, get_string_color_rgb};
use super::DisplayStaged;


/// The `st` command is an abstraction and simplification of some of
/// the most common usages of `git status --porcelain` piped to grep
/// or any other further command-line filtering.
///
/// By default it only displays filenames relative to the current
/// working directory and when called without any flags it defaults to
/// only showing untracked files.
///
/// You can pass the `--desc` (`-D` for short) flag to include the
/// status description of files in the output. The description is
/// never output when stdout is not a tty and a warning is emitted to
/// stderr when the `--desc` flag is used but the output is, say,
/// piped to another program.
///
/// If one or more of the status-related flags is provided, then only
/// display the files whose status match those active flags. For
/// example, calling `git st --deleted` only displays names of deleted
/// and untracked files. To show deleted files which are both
/// untracked and staged, pass the `--staged=matching` option.
#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatusOpt {
    #[arg(short, long, help = "displays new files")]
    added: Option<bool>,

    /// displays deleted files
    #[arg(short, long)]
    deleted: Option<bool>,

    /// displays modified files
    #[arg(short, long)]
    modified: Option<bool>,

    /// defines whether, or how, to display staged files.
    #[arg(short, long)]
    staged: Option<DisplayStaged>,

    /// includes the status description before each file, like `git status` does
    #[arg(short = 'D', long = "desc")]
    description: Option<bool>,
}

impl StatusOpt
{
    pub fn git_repo(&self) -> Result<Repository>
    {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }
}

impl ParserDispatcher<Error> for StatusOpt
{
    fn dispatch(&self) -> Result<()>
    {
        let git = self.git_repo()?;
        Ok(())
    }
}
