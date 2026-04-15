#![allow(unused)]
use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SwitchSharedOpt {
    #[arg(default_value = "~/git_bandolier")]
    path: Path,

}
impl SwitchSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
