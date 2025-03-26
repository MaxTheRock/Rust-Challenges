use std::io::{self, Write};

fn main() {
    let first = input("Enter your first name: ");
    let last = input("Enter your first name: ");

    println!("{first}");
    println!("{last}");
}

fn input(print_text: &str) -> String {
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}