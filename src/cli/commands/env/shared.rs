use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnvSharedOpt {
    #[arg(default_value = "~/workbench")]
    path: Path,

}
impl EnvSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
