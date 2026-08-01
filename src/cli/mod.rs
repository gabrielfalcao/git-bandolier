pub mod commands;
pub use commands::{
    BranchesOpt,
    CommitDatedCommand,
    CommitDatedDirOpt,
    CommitDatedFileOpt,
    CommitDatedOpt,
    CommitDatedSharedOpt,
    MangenOpt,
    PathOpt,
    PathSharedOpt,
    QuickCommitCommand,
    QuickCommitListOpt,
    QuickCommitOpt,
    RemotesOpt,
    SwitchCommand,
    SwitchDirOpt,
    SwitchFileOpt,
    SwitchOpt,
    SwitchSharedOpt,
};

pub mod main;
pub use main::{Cli, Command};

pub(crate) mod traits;
pub use traits::GitRepoAutoDiscover;
