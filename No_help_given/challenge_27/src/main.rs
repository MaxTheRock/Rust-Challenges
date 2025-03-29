fn main() {
    let km: i8 = int(&input("How many kilometers: "));
    let passengers: i8 = int(&input("How many passengers: "));
    let mut total: f32 = float(3 + (2 * (km-1)));

    if passengers >= 5 {
        total *= 1.5;
    }

    println!("Total: £{:.2}", total);
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