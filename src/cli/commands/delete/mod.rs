pub mod opts;
pub use opts::{DeleteDirOpt, DeleteFileOpt};

pub mod shared;
pub use shared::DeleteSharedOpt;

pub mod command;
pub use command::{DeleteCommand, DeleteOpt};
