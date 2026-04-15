#![allow(unused)]
pub mod switch;
pub use switch::{
    SwitchCommand, SwitchDirOpt, SwitchFileOpt, SwitchOpt, SwitchSharedOpt,
};

pub mod web;
pub use web::{WebCommand, WebDirOpt, WebFileOpt, WebOpt, WebSharedOpt};

pub mod commit_dated;
pub use commit_dated::{
    CommitDatedCommand, CommitDatedDirOpt, CommitDatedFileOpt, CommitDatedOpt,
    CommitDatedSharedOpt,
};

pub mod branches;
pub use branches::{
    BranchesCommand, BranchesDirOpt, BranchesFileOpt, BranchesOpt,
    BranchesSharedOpt,
};
