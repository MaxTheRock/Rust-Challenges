fn main() {
    let rect_length: f32 = float(int(&input("How long is your garden (metres): ")));
    let rect_width: f32 = float(int(&input("How wide is your garden (metres): ")));
    println!(" ");
    let circ_radius: f32 = float(int(&input("What's the radius of the flower bed (metres): ")));
    let pi = std::f32::consts::PI;

    let rect_area: f32 = rect_length * rect_width;
    let circ_area: f32 = pi * circ_radius.powi(2);

    println!("Total area: {}m²", rect_area - circ_area);
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