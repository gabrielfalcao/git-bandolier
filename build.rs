use iocore::Path;
use iocore::{Error};


fn main() -> std::result::Result<(), Error>{
    for bin in Path::new("./src/cli/bin").list()? {
        println!("{bin}");
    }
    Ok(())
}
