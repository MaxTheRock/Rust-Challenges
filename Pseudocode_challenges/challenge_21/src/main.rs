fn main() {
    let age: i8 = int(&input("What is your age: "));
    let mut discount: i8;

    if age > 12 && age < 16 {
        discount = 30;
    } else if age > 15 && age < 18 {
        discount = 20;
    } else if age > 49 {
        discount = 40;
    } else {
        discount = 0;
    };

    println!("You have a {}% discount!",discount);
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
