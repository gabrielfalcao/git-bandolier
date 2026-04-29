use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RemotesSharedOpt {
    #[arg(default_value = "~/git_bandolier")]
    path: Path,

}
impl RemotesSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
