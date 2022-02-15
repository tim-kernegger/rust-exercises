use std::io;
// use std::io::{self}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    print!("{buffer}");
    buffer.clear();
    io::stdin().read_line(&mut buffer)?;
    print!("{buffer}");
    buffer.clear();
    io::stdin().read_line(&mut buffer)?;
    print!("{buffer}");
    buffer.clear();
    Ok(())
}
