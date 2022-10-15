use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("What is your name? ");
    io::stdin().read_line(&mut input)?;
    println!("Hello, {}!", input.trim().to_string());
    Ok(())
}