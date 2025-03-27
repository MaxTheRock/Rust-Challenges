use std::io;
use std::io::Write;

fn main() {
    let num1 = int(&input("Enter a number: "));
    let num2 = int(&input("Enter another number: "));

    if num1 > num2 {
        println!("{} is greater than {}",num1,num2);
    } else if num1 < num2 {
        println!("{} is greater than {}",num2,num1);
    } else {
        println!("The numbers are the same")
    };
}

fn input(print_text: &str) -> String {
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn int(string: &str) -> i32 {
    string.parse().expect("Please enter a valid number")
}