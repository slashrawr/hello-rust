use std::io;

fn main() {
    
    let result = prompt();
    let name = match result {
        Ok(entered_name) => entered_name,
        Err(error) => panic!("Error reading name: {:?}", error),
    };
    show(name);
}

fn prompt() -> Result<String, io::Error> {
    let mut input = String::new();
    println!("What is your name? ");
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn show(name: String) {
    println!("Hello, {}!", name.trim().to_string());
}