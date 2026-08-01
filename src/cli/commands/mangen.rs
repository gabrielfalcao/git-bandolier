use std::io::{BufRead, Read, Seek, Write};

use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use clap::{Command, CommandFactory, Parser};
use clap_mangen::Man;
use couleur_rs::{Color, Contrast, Layer};
use git2::{ErrorCode, Oid, Repository};
use iocore::Path;
use slugify_filenames::slugify_string;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Result};

pub(crate) fn valid_directory(val: &str)
-> ::std::result::Result<Path, String>
{
    let path = Path::new(val);

    if path.is_dir() || !path.exists()
    {
        Ok(path)
    }
    else
    {
        Err(format!("output path exists but is not not a directory"))
    }
}
#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MangenOpt
{
    #[arg(
        required = true,
        default_value = "./manpages",
        value_parser,
        help = "path to directory where manfiles will be written to. Should \
                point to either an unexisting path or the path to an existing \
                directory"
    )]
    output_path: Path,

    #[arg(
        short,
        long,
        help = "overwrites any existing files"
    )]
    force: bool,

    #[arg(
        short,
        long,
        help = "does not write any files but instead prints what this command \
                would do without --dry-run",
        conflicts_with = "force"
    )]
    dry_run: bool,
}

impl MangenOpt
{
    pub fn get_current_date_for_man(&self) -> String
    {
        let now = Utc::now();
        let delayed_fmt = now.format("%Y-%m-%d");
        let date_string = format!("{delayed_fmt}");
        date_string
    }

    pub fn get_man(&self, cmd: Command) -> Man
    {
        let man = Man::new(cmd).date(self.get_current_date_for_man());
        man
    }

    // pub fn generate_man<O: Write>(
    //     &self,
    //     cmd: Command,
    //     out: &mut O,
    // ) -> Result<Man>
    // {
    //     let man = Man::new(cmd).date(self.get_current_date_for_man());
    //     man.render(out)?;
    //     Ok(man)
    // }
    pub fn get_cmd_name(&self, cmd: &Command) -> String
    {
        let name = cmd
            .get_bin_name()
            .map(String::from)
            .or_else(|| cmd.get_display_name().map(String::from))
            .unwrap_or_else(|| cmd.get_name().to_string());
        name
    }

    pub fn get_cmd_filename(&self, cmd: &Command) -> Result<String>
    {
        let name = self.get_cmd_name(cmd);
        let slug = slugify_string(name.as_str(), true)?;
        Ok(slug)
    }
}

impl ParserDispatcher<Error> for MangenOpt
{
    fn dispatch(&self) -> Result<()>
    {
        let mut cmd = crate::cli::main::Cli::command_for_update();
        // cmd.set_bin_name("git");

        let output_dir = self.output_path.clone();
        for mut subcmd in cmd.get_subcommands_mut()
        {
            let suffix = self.get_cmd_name(&subcmd);
            if suffix == self.get_cmd_name(&Self::command_for_update())
            {
                continue;
            }
            let filename = format!("git-{suffix}");
            let mut subcmd = subcmd.clone().name(filename.to_string());
            subcmd.set_bin_name(filename.as_str());
            let man = self.get_man(subcmd.clone());
            eprintln!(
                "\x1b[1;38;2;240;79;120mname: \
                 \x1b[1;38;2;48;225;185m{filename}\x1b[0m"
            );
            // let filename = self.get_cmd_filename()?;
            let mut buf = Vec::<u8>::new();
            man.render(&mut buf)?;

            let manpage_file_path = output_dir
                .join("man1")
                .join(filename.as_str())
                .with_extension(".1");
            manpage_file_path.write(&buf)?;

            eprintln!("wrote manual of '{filename}' to '{manpage_file_path}'");
        }
        Ok(())
    }
}
