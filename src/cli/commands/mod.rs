pub mod switch;
pub use switch::{
    SwitchCommand, SwitchDirOpt, SwitchFileOpt, SwitchOpt, SwitchSharedOpt,
};

pub mod web;
pub use web::{WebCommand, WebDirOpt, WebFileOpt, WebOpt, WebSharedOpt};
