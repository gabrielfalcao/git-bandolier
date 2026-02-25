use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GotoSharedOpt {
    #[arg(default_value = "~/workbench")]
    path: Path,

}
impl GotoSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
