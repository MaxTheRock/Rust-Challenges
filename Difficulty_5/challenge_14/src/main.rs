

fn main() {
    let mut choice: String = input("Input a traffic light colour: ") ;
    choice = choice.to_lowercase();

    let output: &str = if choice == "green" {
        "Go"
    } else if choice == "amber" {
        "Get Ready"
    } else {
        "Stop"
    };

    println!("{}",output);
}

fn input(print_text: &str) -> String {
    use std::io::{self,Write};
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}