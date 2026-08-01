#![allow(unused)]
use clap::Parser;
use couleur_rs::{Color, Contrast, Layer};
use git2::Repository;
use iocore::Path;

use crate::{Error, Result, dispatch::ParserDispatcher, get_string_color_rgb};

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RemotesOpt {}

impl RemotesOpt {
    pub fn git_repo(&self) -> Result<Repository> {
        Ok(Repository::discover::<Path>(Path::cwd().into())?)
    }

    pub fn colorize_remote_name(&self, name: &str, reset: bool, algo: Contrast) -> Result<String> {
        let name_color = get_string_color_rgb(name)?;
        let name_color_fg = name_color.to_ansi(Layer::FG, true);
        let name_color_bg = algo.apply(name_color, Layer::BG)?.to_ansi(Layer::BG, true);
        let reset = if reset { "\x1b[0m" } else { "" };
        let ansi_sequence = format!("{reset}{name_color_fg}{name_color_bg}{name}{reset}");
        Ok(ansi_sequence)
    }

    pub fn colorize_remote_url(&self, url: &str, name_color: Color, reset: bool, algo: Contrast) -> Result<String> {
        let name_color_fg = name_color.to_ansi(Layer::BG, true);
        let name_color_bg = algo.apply(name_color, Layer::FG)?.to_ansi(Layer::FG, true);
        let reset = if reset { "\x1b[0m" } else { "" };
        let ansi_sequence = format!("{reset}{name_color_fg}{name_color_bg}{url}{reset}");
        Ok(ansi_sequence)
    }
}

impl ParserDispatcher<Error> for RemotesOpt {
    fn dispatch(&self) -> Result<()> {
        let git = self.git_repo()?;
        let remotes = git
            .remotes()?
            .iter()
            .filter(|name| name.is_some())
            .map(|name| name.unwrap())
            .map(|name| git.find_remote(name))
            .filter(|remote| remote.is_ok())
            .map(|name| name.unwrap())
            .map(|remote| {
                (
                    remote.name().map(|name| name.to_string()),
                    remote.pushurl().map(|url| url.to_string()).or(remote.url().map(|url| url.to_string())),
                )
            })
            .filter(|(name, _url)| name.is_some())
            .map(|(name, url)| (name.unwrap().to_string(), url.unwrap_or_else(|| "<no-url>".to_string()).to_string()))
            .collect::<Vec<(String, String)>>();

        let total = remotes.len();
        let name_max_width = remotes.iter().map(|(name, _)| name.len()).max().unwrap_or_default();
        let url_max_width = remotes.iter().map(|(url, _)| url.len()).max().unwrap_or_default();

        for (index, (name, url)) in remotes.iter().enumerate() {
            let current = index + 1;
            let name_color = get_string_color_rgb(&name)?;
            // println!("[{current: >2} of {total}] {name: >name_max_width$} => {url: >url_max_width$}");
            println!(
                "{colored_name: >name_max_width$} {colored_url: \
                 >url_max_width$}",
                colored_name = self.colorize_remote_name(&name, true, Contrast::Read)?,
                colored_url = self.colorize_remote_url(&url, name_color, true, Contrast::Read)?
            );
        }
        Ok(())
    }
}
