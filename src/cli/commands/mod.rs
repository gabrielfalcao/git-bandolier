pub mod switch;
pub use switch::{
    SwitchCommand, SwitchDirOpt, SwitchFileOpt, SwitchOpt, SwitchSharedOpt,
};

pub mod branches;
pub use branches::BranchesOpt;

pub mod commit_dated;
pub use commit_dated::{
    CommitDatedCommand, CommitDatedDirOpt, CommitDatedFileOpt, CommitDatedOpt,
    CommitDatedSharedOpt,
};

pub mod path;
pub use path::{PathOpt, PathSharedOpt};

pub mod quick_commit;
pub use quick_commit::{
    QuickCommitCommand, QuickCommitListOpt, QuickCommitOpt,
};

pub(crate) mod remotes;
pub use remotes::RemotesOpt;

pub(crate) mod mangen;
pub use mangen::MangenOpt;
