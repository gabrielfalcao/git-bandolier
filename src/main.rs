use clap::Parser;
use git_bandolier::cli::main::Cli;
use git_bandolier::{Exit, ParserDispatcher};

fn main() -> Exit
{
    Cli::main()
}
