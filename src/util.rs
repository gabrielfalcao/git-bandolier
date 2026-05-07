use git2::Repository;
use iocore::Path;

use crate::Result;

pub fn discover_git_repo(starting_point: &Path) -> Result<(Repository, Path)> {
    let home = Path::new("~/").canonicalize()?.to_string();
    let path =
        Repository::discover_path(starting_point, [home.as_str()]).map(|pb| Path::from(pb))?;
    let repo = Repository::open::<Path>(path.clone())?;
    Ok((repo, path))
}
