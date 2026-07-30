#![allow(unused)]
pub mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub mod util;
pub use util::{discover_git_repo, get_string_color_rgb, sha1_hash_hex};

pub mod models;
pub use models::Context;

pub mod cli;
pub use cli::{
    main, BranchesOpt, Cli, Command, CommitDatedCommand, CommitDatedDirOpt,
    CommitDatedFileOpt, CommitDatedOpt, CommitDatedSharedOpt,
    GitRepoAutoDiscover, PathDirOpt, PathFileOpt, PathOpt, PathSharedOpt,
    QuickCommitCommand, QuickCommitListOpt, QuickCommitOpt, RemotesOpt,
    SwitchCommand, SwitchDirOpt, SwitchFileOpt, SwitchOpt, SwitchSharedOpt,
};
