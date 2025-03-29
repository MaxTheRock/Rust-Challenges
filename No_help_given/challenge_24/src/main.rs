use std::collections::HashMap;

fn main() {
    let choices: Vec<&str> = vec!["rock","paper","scissors"];

    let computer_choice: &str = rand_choice(choices);

    let user_choice: &str = &input("Enter [rock-paper-scissors]: ");

    let outcomes = HashMap::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);

    if user_choice == computer_choice {
        println!("It's a draw!");
    } else if let Some(&result) = outcomes.get(user_choice) {
        if result == computer_choice {
            println!("You win!");
            println!("{} beats {}", user_choice, computer_choice);
        } else {
            println!("Computer wins!");
            println!("{} beats {}", computer_choice, user_choice);
        }
    } else {
        println!("Invalid choice!");
    }
}

fn input(print_text: &str) -> String {
    use std::io::{self, Write};
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn rand_choice<T: Clone>(list: Vec<T>) -> T {
    use rand::prelude::*;
    let mut rng = rand::rng();
    list.choose(&mut rng).expect("List cannot be empty").clone()
}
