use std::io::{self, Write};

fn main() {
    let minutes = float(&input("Enter how many minutes you have used in the last month: "));
    let texts = float(&input("Enter how many texts you have used in the last month: "));
    
    let total_minutes_cost = minutes * 0.1;
    let total_texts_cost = texts * 0.05;

    println!("Total bill: ${}", (total_minutes_cost + total_texts_cost) + 10.0);
}

fn input(print_text: &str) -> String {
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn float(string: &str) -> f32 {
    string.parse().expect("Please enter a valid floating-point number")
}
