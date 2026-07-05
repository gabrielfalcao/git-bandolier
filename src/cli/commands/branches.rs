use clap::Parser;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result};
use chrono::{DateTime, Utc};
use couleur::{ContrastAlgorithm, Layer, RGBColor};
use git2::Oid;
use git2::Repository;
use iocore::Path;

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BranchesOpt {}

impl BranchesOpt {
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }
}

impl ParserDispatcher<Error> for BranchesOpt {
    fn dispatch(&self) -> Result<()> {
        let git = self.git_repo()?;
        let mut branches = git
            .branches(Some(git2::BranchType::Local))?
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .map(|(branch, _ty)| {
                let name = branch.name().unwrap().map(|name| name.to_string()).unwrap();
                let commit = branch.into_reference().peel_to_commit().unwrap();
                let commit_hash = commit.id();
                let datetime = DateTime::from_timestamp(commit.time().seconds(), 0).unwrap();
                return NamedBranchInfo {
                    name,
                    commit_hash,
                    datetime,
                };
            })
            .collect::<Vec<NamedBranchInfo>>();

        let max_name_length = branches
            .iter()
            .map(|bi| bi.name.len())
            .max()
            .unwrap_or_default();
        branches.sort_by_key(|info| info.datetime);
        branches.reverse();

        for br in branches {
            println!("{line}", line = br.to_term_line(true, max_name_length)?);
        }

        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NamedBranchInfo {
    pub name: String,
    pub commit_hash: Oid,
    pub datetime: DateTime<Utc>,
}
impl NamedBranchInfo {
    pub fn hash_string(&self) -> String {
        self.commit_hash.to_string()
    }
    pub fn datetime_string(&self) -> String {
        self.datetime.to_string()
    }
    pub fn name_string(&self) -> String {
        self.name.to_string()
    }
    pub fn get_hash_color_rgb(&self) -> Result<RGBColor> {
        let hash = self.hash_string();
        let hash_color = hash.parse::<RGBColor>()?;
        Ok(hash_color)
    }
    pub fn get_hash_color_ansi_sequence(
        &self,
        reset: bool,
        algo: ContrastAlgorithm,
    ) -> Result<String> {
        let hash_color = self.get_hash_color_rgb()?;
        let hash_color_fg = hash_color.to_ansi_sequence(Layer::BG);
        let hash_color_bg = algo.apply(hash_color).to_ansi_sequence(Layer::FG);
        let reset = if reset { "\x1b[0m" } else { "" };
        let ansi_sequence = format!("{reset}{hash_color_fg}{hash_color_bg}");
        Ok(ansi_sequence)
    }
    pub fn get_name_color_ansi_sequence(
        &self,
        reset: bool,
        algo: ContrastAlgorithm,
    ) -> Result<String> {
        let hash_color = self.get_hash_color_rgb()?;
        let name_color_fg = hash_color.to_ansi_sequence(Layer::BG);
        let name_color_bg = algo.apply(hash_color).to_ansi_sequence(Layer::FG);
        let reset = if reset { "\x1b[0m" } else { "" };
        let ansi_sequence = format!("{reset}{name_color_fg}{name_color_bg}");
        Ok(ansi_sequence)
    }
    pub fn to_term_line(&self, colorize: bool, name_width: usize) -> Result<String> {
        let hash = self.hash_string();
        let date = self.datetime_string();
        let name = self.name_string();

        let hash = if colorize {
            let ansi_color = self.get_hash_color_ansi_sequence(true, ContrastAlgorithm::Read)?;
            format!("{ansi_color}{hash}\x1b[0m")
        } else {
            hash
        };
        let name = if colorize {
            let ansi_color = self.get_name_color_ansi_sequence(true, ContrastAlgorithm::Web)?;
            format!("{ansi_color}{name: <name_width$}\x1b[0m")
        } else {
            name
        };
        let date = if colorize {
            let ansi_color = self.get_hash_color_ansi_sequence(true, ContrastAlgorithm::Web)?;
            format!("{ansi_color} {date} \x1b[0m")
        } else {
            date
        };

        Ok(format!("{name}{date}{hash}"))
    }
}
impl std::fmt::Display for NamedBranchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{line}",
            line = self
                .to_term_line(false, self.name.len())
                .expect("branch to string")
        )
    }
}
