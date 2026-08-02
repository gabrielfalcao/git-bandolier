use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use clap::Parser;
use couleur_rs::{Color, Contrast, Layer};
use git2::{ErrorCode, Oid, Repository};
use iocore::Path;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result};

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BranchesOpt
{
    #[arg()]
    new_branch_name: Option<String>,

    #[arg(short, long)]
    force: bool,

    #[arg(
        short,
        long,
        exclusive = true,
        long_help = "prints the current branch name to stdout and exists \
                     without listing branches or doing anything else"
    )]
    print_current: bool,

    #[arg(
        short = 'D',
        long = "delete"
    )]
    delete_branch_name: Option<String>,
}

impl BranchesOpt
{
    pub fn git_repo(&self) -> Result<Repository>
    {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }

    pub fn list_branches(git: &git2::Repository)
        -> Result<Vec<NamedBranchInfo>>
    {
        let mut branches = git
            .branches(Some(git2::BranchType::Local))?
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .map(|(branch, _ty)| {
                let name = branch
                    .name()
                    .unwrap()
                    .map(|name| name.to_string())
                    .unwrap();
                let commit = branch.into_reference().peel_to_commit().unwrap();
                let commit_hash = commit.id();
                let datetime =
                    DateTime::from_timestamp(commit.time().seconds(), 0)
                        .unwrap();
                return NamedBranchInfo {
                    name,
                    commit_hash,
                    datetime,
                };
            })
            .collect::<Vec<NamedBranchInfo>>();
        branches.sort_by_key(|info| info.datetime);
        branches.reverse();
        Ok(branches)
    }

    pub fn display_branch_list(git: &git2::Repository) -> Result<()>
    {
        let branches = Self::list_branches(git)?;
        let max_name_length = branches
            .iter()
            .map(|bi| bi.name.len())
            .max()
            .unwrap_or_default();
        let max_datetime_length = branches
            .iter()
            .map(|bi| bi.datetime_string().len())
            .max()
            .unwrap_or_default();

        for br in branches
        {
            println!(
                "{line}",
                line = br.to_term_line(
                    true,
                    max_name_length,
                    max_datetime_length
                )?
            );
        }
        Ok(())
    }
}

impl ParserDispatcher<Error> for BranchesOpt
{
    fn dispatch(&self) -> Result<()>
    {
        let git = self.git_repo()?;
        if self.print_current
        {
            if git.head_detached()?
            {
                eprintln!("HEAD is detached; not on any branch.");
                std::process::exit(1);
            }
            else
            {
                let head = git.head()?;
                if let Some(branch_name) = head.shorthand()
                {
                    println!("{branch_name}");
                }
                else
                {
                    eprintln!("failed to retrieve current branch name");
                    std::process::exit(1);
                }
            }
        }
        else if self.new_branch_name.is_none()
            && self.delete_branch_name.is_none()
        {
            Self::display_branch_list(&git)?;
        }
        else if let Some(branch_name) = self.new_branch_name.clone()
        {
            // let mut revwalk = git.revwalk()?;
            // revwalk.set_sorting(git2::Sort::NONE | git2::Sort::TIME)?;
            // let commits = revwalk
            //     .filter(|result| result.is_ok())
            //     .map(|result| result.unwrap())
            //     .collect::<Vec<Oid>>();
            // if commits.is_empty() {
            //     eprintln!("cannot create branch: no commits found");
            //     std::process::exit(1);
            // }
            let head = git.head()?;
            let commit_oid = head.target().expect("git HEAD");
            let commit = git.find_commit(commit_oid)?;
            match git.branch(&branch_name, &commit, self.force)
            {
                Ok(branch) =>
                {
                    println!("created branch {branch_name}");
                },
                Err(error) => match error.code()
                {
                    ErrorCode::Exists =>
                    {
                        eprintln!("branch {branch_name:#?} already exists");
                    },
                    _code =>
                    {
                        eprintln!(
                            "failed to create branch {branch_name:#?}: {error}"
                        );
                    },
                },
            }
        }
        else if let Some(branch_name) = self.delete_branch_name.clone()
        {
            match git.find_branch(&branch_name, git2::BranchType::Local)
            {
                Ok(mut branch) => match branch.delete()
                {
                    Ok(_) =>
                    {
                        println!("deleted branch {branch_name:#?}");
                    },
                    Err(error) => match error.code()
                    {
                        ErrorCode::NotFound =>
                        {
                            eprintln!("branch {branch_name:#?} not found");
                        },
                        code =>
                        {
                            eprintln!("{error} (code: {code:#?})");
                        },
                    },
                },
                Err(error) => match error.code()
                {
                    ErrorCode::NotFound =>
                    {
                        eprintln!("branch {branch_name:#?} not found");
                    },
                    code =>
                    {
                        eprintln!("{error} (code: {code:#?})");
                    },
                },
            }
            // delete branch
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NamedBranchInfo
{
    pub name: String,
    pub commit_hash: Oid,
    pub datetime: DateTime<Utc>,
}
impl NamedBranchInfo
{
    pub fn hash_string(&self) -> String
    {
        self.commit_hash.to_string()
    }

    pub fn datetime_string(&self) -> String
    {
        let human = HumanTime::from(self.datetime.clone());
        human.to_string()
    }

    pub fn name_string(&self) -> String
    {
        self.name.to_string()
    }

    pub fn get_hash_color_rgb(&self) -> Result<Color>
    {
        let hash = self.hash_string();
        let mut hash_color = hash.parse::<Color>()?;
        let mut shift = 0;
        while hash_color.is_dark() && shift < (hash.len() - 6)
        {
            shift += 1;
            hash_color = hash[shift..shift + 6].parse::<Color>()?;
        }
        Ok(hash_color)
    }

    pub fn get_hash_color_ansi_sequence(
        &self,
        reset: bool,
        algo: Contrast,
    ) -> Result<String>
    {
        let hash_color = self.get_hash_color_rgb()?;
        let hash_color_fg = hash_color.to_ansi(Layer::BG, true);
        let hash_color_bg = algo
            .apply(hash_color, Layer::BG)?
            .to_ansi(Layer::FG, true);
        let reset = if reset
        {
            "\x1b[0m"
        }
        else
        {
            ""
        };
        let ansi_sequence = format!("{reset}{hash_color_fg}{hash_color_bg}");
        Ok(ansi_sequence)
    }

    pub fn get_name_color_ansi_sequence(
        &self,
        reset: bool,
        algo: Contrast,
    ) -> Result<String>
    {
        let hash_color = self.get_hash_color_rgb()?;
        let name_color_fg = hash_color.to_ansi(Layer::FG, true);
        let name_color_bg = algo
            .apply(hash_color, Layer::BG)?
            .to_ansi(Layer::BG, true);
        let reset = if reset
        {
            "\x1b[0m"
        }
        else
        {
            ""
        };
        let ansi_sequence = format!("{reset}{name_color_fg}{name_color_bg}");
        Ok(ansi_sequence)
    }

    pub fn to_term_line(
        &self,
        colorize: bool,
        name_width: usize,
        date_width: usize,
    ) -> Result<String>
    {
        let hash = self.hash_string();
        let date = self.datetime_string();
        let name = self.name_string();

        let hash = if colorize
        {
            let ansi_color =
                self.get_hash_color_ansi_sequence(true, Contrast::Read)?;
            format!("{ansi_color}{hash}\x1b[0m")
        }
        else
        {
            hash
        };
        let name = if colorize
        {
            let ansi_color =
                self.get_name_color_ansi_sequence(true, Contrast::Read)?;
            format!("{ansi_color}{name: <name_width$}\x1b[0m")
        }
        else
        {
            name
        };
        let date = if colorize
        {
            let ansi_color =
                self.get_name_color_ansi_sequence(true, Contrast::Read)?;
            format!("{ansi_color} {date: <date_width$} \x1b[0m")
        }
        else
        {
            date
        };

        Ok(format!("{name}{date}{hash}"))
    }
}
impl std::fmt::Display for NamedBranchInfo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "{line}",
            line = self
                .to_term_line(
                    false,
                    self.name.len(),
                    self.datetime_string().len()
                )
                .expect("branch to string")
        )
    }
}
