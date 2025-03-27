use rand::Rng;
use std::io::{self, Write};

fn main() {
    let random_num: i8 = rand_int(1,11);
    let guess: i8 = int(&input("Enter a number between [1-10]: "));

    if random_num == guess {
        println!("Correct");
    } else {
        println!("Not what I was thinking.");
    }
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

fn rand_int(min: i8, max: i8) -> i8 {
    let num = rand::rng().random_range(min..max);
    num
}
