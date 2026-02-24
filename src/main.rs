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
    #[arg()]
    text: Vec<String>,
}
impl Cli {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
}
impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        println!("{}", &self.text());

        Ok(())
    }
}


fn main() -> Exit {
    Cli::main()
}
