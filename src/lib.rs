pub mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub mod cli;
pub use cli::{
    BootstrapCommand, BootstrapSharedOpt, BootstrapDirOpt, BootstrapFileOpt,
    BootstrapOpt,
};
