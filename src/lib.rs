pub mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub mod util;
pub use util::discover_git_repo;

pub mod models;
pub use models::Context;

pub mod cli;
pub use cli::{
    BranchesOpt, CommitDatedCommand, CommitDatedDirOpt, CommitDatedFileOpt, CommitDatedOpt,
    CommitDatedSharedOpt, PathDirOpt, PathFileOpt, PathOpt, PathSharedOpt, QuickCommitCommand,
    QuickCommitListOpt, QuickCommitOpt, RemotesCommand, RemotesOpt, SwitchCommand, SwitchDirOpt,
    SwitchFileOpt, SwitchOpt, SwitchSharedOpt,
};

// pub mod cli;
// pub use cli::{
//     CommitDatedCommand,   // CommitDatedCommand
//     CommitDatedDirOpt,    // CommitDatedDirOpt
//     CommitDatedFileOpt,   // CommitDatedFileOpt
//     CommitDatedOpt,       // CommitDatedOpt
//     CommitDatedSharedOpt, // CommitDatedSharedOpt

//     SwitchCommand, // SwitchCommand
//     SwitchDirOpt,  // SwitchDirOpt
//     SwitchFileOpt, // SwitchFileOpt
//     SwitchOpt,     // SwitchOpt

//     PathCommand, // PathCommand
//     PathDirOpt,  // PathDirOpt
//     PathFileOpt, // PathFileOpt
//     PathOpt,     // PathOpt

//     WebCommand,   // WebCommand
//     WebDirOpt,    // WebDirOpt
//     WebFileOpt,   // WebFileOpt
//     WebOpt,       // WebOpt
//     WebSharedOpt, // WebSharedOpt

//     BranchesOpt,     // BranchesOpt

// };
