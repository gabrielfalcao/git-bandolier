#![allow(unused)]
use clap::{Parser, Subcommand};
use git_bandolier::cli::commands::{SwitchOpt, WebOpt, CommitDatedOpt};
use git_bandolier::dispatch::{
    ArgsDispatcher, ParserDispatcher, SubcommandDispatcher,
};
use git_bandolier::{Error, Exit, Result};

fn main() -> Exit {
    CommitDatedOpt::main()
}
