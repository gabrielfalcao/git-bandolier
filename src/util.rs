use couleur_rs::{Color, Contrast, Layer};
use git2::Repository;
use iocore::Path;
use sha1::{Digest, Sha1};

use crate::Result;

pub fn discover_git_repo(starting_point: &Path) -> Result<(Repository, Path)>
{
    let home = Path::new("~/").canonicalize()?.to_string();
    let path = Repository::discover_path(starting_point, [home.as_str()])
        .map(|pb| Path::from(pb))?;
    let repo = Repository::open::<Path>(path.clone())?;
    Ok((repo, path))
}

pub fn sha1_hash_hex(data: &[u8]) -> String
{
    let result = Sha1::digest(data);
    hex::encode(result)
}

pub fn get_string_color_rgb(string: &str) -> Result<Color>
{
    let hash = sha1_hash_hex(string.as_bytes());
    let mut hash_color = hash.parse::<Color>()?;
    let mut shift = 0;
    while hash_color.is_dark() && shift < (hash.len() - 6)
    {
        shift += 1;
        hash_color = hash[shift..shift + 6].parse::<Color>()?;
    }
    Ok(hash_color)
}
