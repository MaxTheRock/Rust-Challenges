use std::collections::HashMap;

fn main() {
    let deposit: i16 = int(&input("Enter deposit amount: "));
    let no_banks: i16 = int(&input("Enter number of banks: "));
    let years: f32 = ifloat(int(&input("How many years: ")));
    let mut banks: HashMap<String, f32> = HashMap::new();

    for i in 1..(no_banks + 1) {
        print!("Bank {} - ",i);
        let interest: f32 = float(&input("Enter interest rate: "));
        banks.insert(format!("Bank {}", i), interest);
    }

    let mut best_bank: String = "N/A".to_string();

    let mut best: f32 = 0.0;
    for i in 1..(no_banks + 1) {
        let key = format!("Bank {}", i);
        if let Some(&interest) = banks.get(&key) {
            if interest > best {
                best = interest;
                best_bank = key.clone();
            }
        }
    }
    println!("------------------------------------------");
    println!("Best bank: {}", best_bank);
    println!("Total savings: £{:.2}",years * (best * ifloat(deposit)));
    println!("Profit per year: £{:.2}",(best * ifloat(deposit)) - ifloat(deposit));
    println!("------------------------------------------");
}

fn input(print_text: &str) -> String {
    use std::io::{self, Write};
    print!("{}", print_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn int(string: &str) -> i16 {
    string.parse().expect("Please enter a valid number")
}

fn float(string: &str) -> f32 {
    string.parse().expect("Please enter a valid floating-point number")
}

fn ifloat(int: i16) -> f32 {
    int as f32
}