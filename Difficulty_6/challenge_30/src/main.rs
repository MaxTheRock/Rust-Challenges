fn main() {
    let side1: f32 = float(int(&input("Enter side 1: ")));
    let side2: f32 = float(int(&input("Enter side 2: ")));
    let side3: f32 = float(int(&input("Enter side 3: ")));

    if side1 == side2 || side2 == side3 || side3 == side1 {
        println!("The triangle is isosceles!");
    } else {
        println!("The triangle isn't isosceles!")
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

fn float(int: i8) -> f32 {
    int as f32
}