use iocore::Error;
use iocore::Path;

fn main() -> std::result::Result<(), Error> {
    for bin in Path::new("./src/bin").list()? {
        println!("{bin}");
    }
    Ok(())
}
