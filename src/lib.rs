pub mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub mod models;
pub use models::Context;

pub mod cli;
pub use cli::{
    CommitDatedCommand,   // CommitDatedCommand
    CommitDatedDirOpt,    // CommitDatedDirOpt
    CommitDatedFileOpt,   // CommitDatedFileOpt
    CommitDatedOpt,       // CommitDatedOpt
    CommitDatedSharedOpt, // CommitDatedSharedOpt

    SwitchCommand, // SwitchCommand
    SwitchDirOpt,  // SwitchDirOpt
    SwitchFileOpt, // SwitchFileOpt
    SwitchOpt,     // SwitchOpt

    WebCommand,   // WebCommand
    WebDirOpt,    // WebDirOpt
    WebFileOpt,   // WebFileOpt
    WebOpt,       // WebOpt
    WebSharedOpt, // WebSharedOpt

    BranchesOpt,     // BranchesOpt

};
