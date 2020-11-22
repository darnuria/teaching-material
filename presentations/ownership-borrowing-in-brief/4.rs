use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("test")?;

    print_filelen(&file)?;
    file.write_all(b"Hello World!")?;
    print_filelen(&file)
}

fn print_filelen(f: &File) -> io::Result<()> {
    println!("len: {:?}", f.metadata()?.len());
    Ok(())
}
