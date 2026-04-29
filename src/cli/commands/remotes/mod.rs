pub mod command;
pub use command::{RemotesCommand, RemotesOpt};

pub mod shared;
pub use shared::RemotesSharedOpt;

pub mod opts;
pub use opts::{RemotesDirOpt, RemotesFileOpt};
