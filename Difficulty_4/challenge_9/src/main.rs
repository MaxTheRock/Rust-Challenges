use std::io;
use std::io::Write;

fn main() {
    let random_name = "Max";
    let name = input("What's your name: ");

    if name == random_name {
        println!("You're cool.");
    } else {
        println!("Nice to meet you.");
    };
}

fn input(print_text: &str) -> String {
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}