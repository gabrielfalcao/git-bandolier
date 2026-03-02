use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeleteSharedOpt {
    #[arg(default_value = "~/workbench")]
    path: Path,

}
impl DeleteSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
