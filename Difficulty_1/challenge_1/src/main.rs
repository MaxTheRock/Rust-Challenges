use std::io::{self, Write};

fn main() {
    let _name = input("Enter your name: ");
    let _age = int(&input("Enter your age: "));
    let _fav_color = input("Enter your favourite colour: ");
}

fn input(print_text: &str) -> String {
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn int(string: &str) -> i32 {
    string.parse().expect("Please enter a valid number")
}