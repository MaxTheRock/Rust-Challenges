use std::io;
use std::io::Write;

fn main() {
    let alphabet = 26;
    let choice = int(&input("How many letters are in the alphabet: "));

    if choice == alphabet {
        println!("Correct");
    } else {
        println!("No, there are 26 letters!");
    };
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