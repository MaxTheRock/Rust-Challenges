use std::io::{self, Write};

fn main() {
    let _holiday_time: i8 = 28;
    let choice: String = input("Are you working full time: ");

    let full_time: bool = if choice.to_lowercase() == "yes" {true} else {false};
    let work_days: i8 = int(&input("How many days do you work a week: "))
    
}

fn input(print_text: &str) -> String {
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn int(string: &str) -> i8 {
    string.parse().expect("Please enter a valid number")
}
