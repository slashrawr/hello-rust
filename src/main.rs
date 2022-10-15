use std::io;

fn main() {
    let result = prompt();
    let name = match result {
        Ok(entered_name) => entered_name,
        Err(error) => panic!("Error reading name: {:?}", error),
    };
    println!("{}", build_output(name));
}

fn prompt() -> Result<String, io::Error> {
    let mut input = String::new();
    println!("What is your name? ");
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn build_output(name: String) -> String {
    let output = format!("Hello {}!", name);
    return output;
}

#[cfg(test)]
mod tests {
    use super::build_output;

    #[test]
    fn formatted_output() {
        let result  = build_output("Brandon".to_string());
        assert!(result.contains("Hello Brandon!"))
    }
}