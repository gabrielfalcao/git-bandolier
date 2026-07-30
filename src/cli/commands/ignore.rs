use std::collections::BTreeSet;

use chrono::{DateTime, Local};
use clap::{Args, Parser, Subcommand};
use iocore::Path;
use regex::Regex;

use crate::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use crate::{Error, GitRepoAutoDiscover, Result};

#[derive(Parser, Debug, Clone)]
pub struct IgnoreOpt
{
    #[command(subcommand)]
    command: GitIgnoreCommand,
}
impl ParserDispatcher<Error> for IgnoreOpt
{
    fn dispatch(&self) -> Result<()>
    {
        self.command.dispatch()?;
        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum GitIgnoreCommand
{
    Add(GitIgnoreAddOpt),
    Remove(GitIgnoreRemoveOpt),
    Rm(GitIgnoreRemoveOpt),
}
impl SubcommandDispatcher<Error> for GitIgnoreCommand
{
    fn dispatch(&self) -> Result<()>
    {
        match self
        {
            GitIgnoreCommand::Add(op) => op.dispatch()?,
            GitIgnoreCommand::Remove(op) => op.dispatch()?,
            GitIgnoreCommand::Rm(op) => op.dispatch()?,
        }

        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct GitIgnoreAddOpt
{
    #[arg()]
    pub patterns: Vec<String>,

    #[arg(short, long)]
    pub no_autoignore: bool,

    #[arg(
        short,
        long,
        help = "write changes to .gitignore instead of printing to stdout"
    )]
    pub write: bool,

    #[arg(
        short,
        long,
        help = "path to be used as starting point to discover the git repo \
                path, defaults to the current working directory"
    )]
    pub starting_point: Option<Path>,
}
impl GitIgnoreAddOpt
{
    pub fn patterns(&self) -> Vec<String>
    {
        let mut result = Vec::<String>::new();
        for pat in self.patterns.iter().map(|pat| pat.trim().to_string())
        {
            if !result.contains(&pat)
            {
                result.push(pat.to_string());
            }
        }
        if !self.no_autoignore
        {
            for path in self.git_status_list_untracked_paths()
            {
                let pat = path.to_string();
                if !result.contains(pat)
                {
                    result.push(pat);
                }
            }
        }
        result
    }
}
impl GitRepoAutoDiscover for GitIgnoreAddOpt
{
    fn starting_point(&self) -> Path
    {
        self.chdir.clone().unwrap_or_else(|| Path::cwd())
    }
}
impl ArgsDispatcher<Error> for GitIgnoreAddOpt
{
    fn dispatch(&self) -> Result<()>
    {
        let git_ignore_path = self.git_ignore_path()?;

        let mut lines: BTreeSet<String> = if git_ignore_path.is_file()
        {
            git_ignore_path
                .read_lines()?
                .into_iter()
                .collect::<BTreeSet<String>>()
        }
        else if !git_ignore_path.exists()
        {
            BTreeSet::<String>::new()
        }
        else
        {
            return Err(Error::IOError(format!(
                ".gitignore exists but is not a regular file: \
                 '{git_ignore_path}'"
            )));
        };

        let now = Local::now();
        let banner = format!(
            "# added by git-bandolier via {argv} on {now}:",
            argv = iocore::env::args().join(" ")
        );
        let patterns = self.patterns();
        for (index, pattern) in patterns.iter().enumerate()
        {
            if !lines.contains(&pattern)
            {
                if index == 0
                {
                    lines.push(String::new());
                    lines.push(banner.to_string());
                }
                lines.push(pattern.to_string());
            }
        }
        if patterns.len() > 0
        {
            lines.push(String::new());
        }

        let new_content = lines.join("\n").to_string();
        if self.write
        {
            git_ignore_path.write(new_content.as_bytes())?;
        }
        else
        {
            println!("{new_content}");
        }

        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct GitIgnoreRemoveOpt
{
    #[arg()]
    pub patterns: Vec<String>,

    #[arg(
        short,
        long,
        help = "write changes to .gitignore instead of printing to stdout"
    )]
    pub write: bool,

    #[arg(
        short,
        long,
        help = "path to be used as starting point to discover the git repo \
                path, defaults to the current working directory"
    )]
    pub starting_point: Option<Path>,
}
impl GitIgnoreRemoveOpt
{
    pub fn patterns(&self) -> Vec<String>
    {
        self.patterns.clone()
    }
}
impl GitRepoAutoDiscover for GitIgnoreRemoveOpt
{
    fn starting_point(&self) -> Path
    {
        self.chdir.clone().unwrap_or_else(|| Path::cwd())
    }
}
pub enum MatchStrategy
{
    String,
    FnMatch,
    Regex,
}

/// `pattern_with_strategies` tries to match `pattern` against `haystack` and returns some tuple
/// with the [`MatchStrategy`] and a string with pattern which
/// succeeded match
pub fn match_pattern_with_strategies(
    pattern_str: &str,
    haystack: &str,
) -> Option<(MatchStrategy, String)>
{
    if haystack.to_string() == haystack.to_string()
    {
        return Some((MatchStrategy::String, pattern_str.to_string()));
    }
    if let Ok(regex) = fnmatch_regex::glob_to_regex(pattern_str)
    {
        if let Some(found) = regex.find(haystack)
        {
            return Some((MatchStrategy::FnMatch, found.as_str().to_string()));
        }
    }
    if let Ok(regex) = Regex::new(pattern_str)
    {
        if let Some(found) = regex.find(haystack)
        {
            return Some((MatchStrategy::Regex, found.as_str().to_string()));
        }
    }
    None
}
impl ArgsDispatcher<Error> for GitIgnoreRemoveOpt
{
    fn dispatch(&self) -> Result<()>
    {
        let git_ignore_path = self.git_ignore_path()?;

        let mut lines: BTreeSet<String> = if git_ignore_path.is_file()
        {
            git_ignore_path
                .read_lines()?
                .into_iter()
                .collect::<BTreeSet<String>>()
        }
        else if !git_ignore_path.exists()
        {
            BTreeSet::<String>::new()
        }
        else
        {
            return Err(Error::IOError(format!(
                ".gitignore exists but is not a regular file: \
                 '{git_ignore_path}'"
            )));
        };

        for pat_raw in self.patterns()
        {
            let pat_fnmatch = fnmatch_regex::glob_to_regex(&pat_raw);
            let pat_regexp = Regex::new(&pat_raw);

            let mut indexes_to_delete = lines
                .iter()
                .map(|haystack| {
                    match_pattern_with_strategies(
                        pat_raw.as_str(),
                        haystack.as_str(),
                    )
                })
                .enumerate()
                .filter(|(index, matched)| matched.is_some())
                .collect::<Vec<usize>>();
            let mut removed_lines = Vec::<String>::new();
            while indexes_to_delete.len() > 0
            {
                let index = indexes_to_delete.pop();
                removed_lines.push(lines.remove(&index).to_string());
            }
        }

        let new_content = lines.into_iter().collect::<Vec<String>>().join("\n").to_string();
        if self.write
        {
            git_ignore_path.write(new_content.as_bytes())?;
        }
        else
        {
            println!("{new_content}");
        }

        Ok(())
    }
}
