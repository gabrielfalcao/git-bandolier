use clap::Args;
use iocore::Path;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WebSharedOpt {
    #[arg(default_value = "~/git_bandolier")]
    path: Path,

}
impl WebSharedOpt {
    pub fn path(&self) -> Path {
        self.path.try_canonicalize()
    }
}
