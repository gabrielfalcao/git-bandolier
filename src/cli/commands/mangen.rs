use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use clap::Parser;
use couleur_rs::{Color, Contrast, Layer};
use git2::{ErrorCode, Oid, Repository};
use iocore::Path;

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
    #[arg(required = True, value_parser, help="path to directory where manfiles will be written to. Should point to either an unexisting path or the path to an existing directory")]
    output_path: Path,

    #[arg(short, long, help = "overwrites any existing files")]
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

    pub fn generate_man<T: Command, O: Write>(
        &self,
        cmd: T,
        &mut out: O,
    ) -> Result<()>
    {
        Ok(Man::new(cmd).date(self.get_current_date_for_man()).render(out)?)
    }
}

impl ParserDispatcher<Error> for MangenOpt
{
    fn dispatch(&self) -> Result<()>
    {
        let cmd = crate::cli::main::Command::new();
        generate_man(cmd)?;
        Ok(())
    }
}
