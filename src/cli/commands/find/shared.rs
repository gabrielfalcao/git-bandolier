use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FindSharedOpt {
    #[arg(default_value = "~/workbench")]
    path: Path,

}
impl FindSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
