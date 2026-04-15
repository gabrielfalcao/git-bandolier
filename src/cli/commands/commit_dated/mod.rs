#![allow(unused)]
pub mod opts;
pub use opts::{CommitDatedDirOpt, CommitDatedFileOpt};

pub mod shared;
pub use shared::CommitDatedSharedOpt;

pub mod command;
pub use command::{CommitDatedCommand, CommitDatedOpt};
