fn main() {
    let amount: i16 = int(&input("How many fruits do you have: "));
    let mut total: i16 = 0;
    for _ in 1..(amount+1) {
        total += int(&input("Enter the weight of the fruit: "));
    };

    println!("Average weight: {}", total / amount);
}

fn input(print_text: &str) -> String {
    use std::io::{self, Write};
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn int(string: &str) -> i16 {
    string.parse().expect("Please enter a valid number")
}