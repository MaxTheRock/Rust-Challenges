fn main() {
    println!("Dog -> Human age calculator!");

    let dog_age: i16 = int(&input("What is the dog's age: "));
    let human_age: i16;

    if dog_age <= 2 {
        human_age = dog_age * 12;
    } else if dog_age > 2 {
        human_age = 24 + (6 * (dog_age - 2));
    } else {
        human_age = 0;
    };

    println!("Dog years: {}", dog_age);
    println!("Human years: {}",human_age);
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