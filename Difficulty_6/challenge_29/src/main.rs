fn main() {
    let made: i8 = int(&input("How many teddy bears have you made: "));
    let hours: i8 = int(&input("How many hours have you worked for: "));

    if (made * 2) > (hours * 5) {
        println!("Wage: £{:.2}", float(made * 2));
    } else {
        println!("Wage: £{:.2}", float(hours * 5));
    };
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