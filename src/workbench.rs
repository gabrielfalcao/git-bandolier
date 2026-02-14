use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Workbench {}

impl Workbench {
    pub fn display_output(text: impl std::fmt::Display) -> Result<()> {
        println!("{text}");
        Ok(())
    }

    pub fn display_error(error: Error) {
        eprintln!("{error}");
    }
}
