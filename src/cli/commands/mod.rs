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

// TODO: fix errors and integrate `ignore` module (less than 15 errors remaining)
// pub(crate) mod ignore;
// pub use ignore::{GitIgnoreCommand, IgnoreOpt};

pub(crate) mod mangen;
pub use mangen::MangenOpt;
