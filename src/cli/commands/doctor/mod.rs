pub mod opts;
pub use opts::{DoctorDirOpt, DoctorFileOpt};

pub mod shared;
pub use shared::DoctorSharedOpt;

pub mod command;
pub use command::{DoctorCommand, DoctorOpt};
