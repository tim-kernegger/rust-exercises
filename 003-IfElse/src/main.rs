use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let num: u8 = buffer.trim().parse::<u8>().unwrap();
    if num % 2 == 1 {
        print!("Weird");
    } else {
        if num >= 2 && num <= 5 {
            print!("Not Weird");
        } else if num >= 6 && num <= 20 {
            print!("Weird");
        } else {
            print!("Not Weird");
        }
    }
    Ok(())
}
