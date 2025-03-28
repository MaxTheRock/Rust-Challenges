use std::io::{self, Write};

fn main() {
    let choice: String = input("Are you working full time or part time: ");

    let full_time: bool = if choice.to_lowercase() == "full" {
        true
    } else  {
        false
    };
    let work_days: f32 = float(int(&input("How many days do you work a week: ")));
    let mut holiday_time: f32 = 0.0;
    if full_time && work_days == 5.0 {
        holiday_time = 28.0;
    } else if !full_time {
        let multiplyer: f32 = work_days / 5.0;
        holiday_time = 28.0 * multiplyer;
    };

    println!("Holiday time allowed: {} days",holiday_time);
    
}

fn input(print_text: &str) -> String {
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
