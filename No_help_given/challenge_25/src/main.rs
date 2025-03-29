fn main() {
    // Age groups
    let mut under_18: Vec<i8> = vec![];
    let mut adults: Vec<i8> = vec![];
    let mut seniors: Vec<i8> = vec![];

    println!("Welcome to SpeedyClub Runners!");
    let total_people: i8 = int(&input("How many runners are taking part in the 5k race: "));

    for i in 1..(total_people + 1) {
        print!("{}. ",i);
        let runner_age: i8 = int(&input("How old is the runner: "));

        if runner_age < 18 {
            under_18.push(i);
        } else if runner_age > 64 {
            seniors.push(i);
        } else {
            adults.push(i);
        };
    };

    println!("Under 18's: {:?}  Total: {}", under_18, under_18.len());
    println!("Adults: {:?}  Total: {}", adults, adults.len());
    println!("Seniors: {:?}  Total: {}", seniors, seniors.len());
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