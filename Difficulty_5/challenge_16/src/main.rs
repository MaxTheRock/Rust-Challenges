fn main() {
    let time: i8 = int(&input("How long on average do you spend watching TV: "));

    if time < 2 {
        println!("That should be ok.");
    } else if time <= 4 {
        println!("That will rot your brain");
    } else {
        println!("That is too much TV")
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