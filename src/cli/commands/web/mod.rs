pub mod opts;
pub use opts::{WebDirOpt, WebFileOpt};

pub mod shared;
pub use shared::WebSharedOpt;

pub mod command;
pub use command::{WebCommand, WebOpt};
