fn main() {
    let values: Vec<&str> = vec!["respect", "excellence", "friendship"];
    let user_input = input("Enter an Olympic Value: ");
    
    if values.contains(&user_input.as_str()) {
        println!("That's correct");
    } else {
        println!("Incorrect");
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
