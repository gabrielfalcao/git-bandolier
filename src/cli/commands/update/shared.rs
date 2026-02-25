use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpdateSharedOpt {
    #[arg(default_value = "~/workbench")]
    path: Path,

}
impl UpdateSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
