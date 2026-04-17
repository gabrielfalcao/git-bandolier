use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathSharedOpt {
    #[arg(default_value = "~/git_bandolier")]
    path: Path,

}
impl PathSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
