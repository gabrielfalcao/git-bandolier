pub mod command;
pub use command::{SwitchCommand, SwitchOpt};

pub mod shared;
pub use shared::SwitchSharedOpt;

pub mod opts;
pub use opts::{SwitchDirOpt, SwitchFileOpt};
