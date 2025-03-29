fn main() {
    let scores: Vec<i8> = vec![1,3,5,7,9,11,13,15,17,20];
    let grades: Vec<i8> = vec![0,1,2,3,4,5,6,7,8,9];

    let score = int(&input("What score did you get on the test [0-20]: "));

    for i in 0..10 {
        if score <= 1 {
            println!("Your grade is: U");
            break;
        } else if !(score > scores[i]) {
            println!("Your grade is: {}", grades[i]);
            break;
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
