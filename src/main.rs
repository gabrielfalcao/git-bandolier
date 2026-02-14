use clap::{Parser, Subcommand};
use workbench::dispatch::{
    ArgsDispatcher, ParserDispatcher, SubcommandDispatcher,
};
use workbench::{Error, Exit, Result};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "workbench command-line"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Server(ServerOpt),

    Cli(CliOpt),
}
impl SubcommandDispatcher<Error> for Command {
    fn dispatch(&self) -> Result<()> {
        match self {
            Command::Server(op) => op.dispatch()?,

            Command::Cli(op) => op.dispatch()?,
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct ServerOpt {
    #[arg()]
    text: Vec<String>,
}
impl ServerOpt {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
}
impl ArgsDispatcher<Error> for ServerOpt {
    fn dispatch(&self) -> Result<()> {
        println!("{}", &self.text());

        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct CliOpt {
    #[arg()]
    text: Vec<String>,
}
impl CliOpt {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
}
impl ArgsDispatcher<Error> for CliOpt {
    fn dispatch(&self) -> Result<()> {
        println!("{}", &self.text());

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
