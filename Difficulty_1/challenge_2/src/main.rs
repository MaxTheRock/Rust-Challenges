use std::io::{self, Write};

fn main() {
    let name = input("Enter your first name: ");

    println!("{name}")
}

fn input(print_text: &str) -> String {
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}