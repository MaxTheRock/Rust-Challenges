fn main() {
    loop {
        let guess: i8 = int(&input("Enter a number: "));
    
        if guess == 7 {
            println!("Well Done");
            break;
        } else {
            println!("Nope, try again!");
        };
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