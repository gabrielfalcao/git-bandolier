use clap::Parser;
use git_bandolier::{Exit, ParserDispatcher, cli::main::Cli};

fn main() -> Exit {
    Cli::main()
}
