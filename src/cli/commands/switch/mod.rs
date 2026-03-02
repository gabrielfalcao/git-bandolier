pub mod opts;
pub use opts::{SwitchDirOpt, SwitchFileOpt};

pub mod shared;
pub use shared::SwitchSharedOpt;

pub mod command;
pub use command::{SwitchCommand, SwitchOpt};
