use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShellSharedOpt {
    #[arg(default_value = "~/workbench")]
    path: Path,

}
impl ShellSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
