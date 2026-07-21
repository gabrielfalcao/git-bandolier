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
    BranchesOpt, CommitDatedCommand, CommitDatedDirOpt, CommitDatedFileOpt,
    CommitDatedOpt, CommitDatedSharedOpt, PathDirOpt, PathFileOpt, PathOpt,
    PathSharedOpt, QuickCommitCommand, QuickCommitListOpt, QuickCommitOpt,
    RemotesOpt, SwitchCommand, SwitchDirOpt, SwitchFileOpt, SwitchOpt,
    SwitchSharedOpt,
};
