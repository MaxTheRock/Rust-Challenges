fn main() {
    let money_input: f32 = float(int(&input("How much pounds do you want to change to coins: ")));

    println!("Which coin do you want to convert to money: ");
    println!("£1, 50p, 20p, 10p, 5p, 2p, 1p");
    let choice: &str = &input("  > ");
    let coin_type: f32 = if choice == "£1" {
        1.0
    } else if choice == "50p" {
        0.5
    } else if choice == "20p" {
        0.2
    } else if choice == "10p" {
        0.1
    } else if choice == "5p" {
        0.05
    } else if choice == "2p" {
        0.02
    } else {
        0.01
    };

    println!("With £{}, you can have {} {} coins!", money_input, money_input / coin_type, choice);
}
fn input(print_text: &str) -> String {
    use std::io::{self, Write};
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn int(string: &str) -> i8 {
    string.parse().expect("Please enter a valid number")
}

fn float(int: i8) -> f32 {
    int as f32
}
