pub mod commands;

pub use commands::switch::{
    SwitchCommand, SwitchSharedOpt, SwitchDirOpt, SwitchFileOpt,
    SwitchOpt,
};


pub use commands::web::{
    WebCommand, WebSharedOpt, WebDirOpt, WebFileOpt,
    WebOpt,
};
