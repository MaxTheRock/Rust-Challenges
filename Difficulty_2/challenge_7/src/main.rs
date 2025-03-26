use std::io::{self, Write};

fn main() {
    let distance = int(&input("Enter distance: "));
    let time = int(&input("Enter time: "));
    println!("Average Speed: {}", distance / time);
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