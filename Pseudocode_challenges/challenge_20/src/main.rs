fn main() {
    let mut card_list: Vec<String> = vec![];
    let mut ordered: i8 = 0;
    let mut total_pizzas: i8 = 0;
    let name: String = input("What's your name: ");
    loop {
        ordered = int(&input("How many pizzas have you ordered: "));
        total_pizzas = total_pizzas + ordered;

        if total_pizzas > 20 && !card_list.contains(&name) {
            card_list.push(name.clone());
            println!("Added {} to the loyalty reward system!", &name);
        } else if total_pizzas <= 20 && card_list.contains(&name) {
            card_list.retain(|x| x != &name);
            println!("Removed {} to the loyalty reward system!", &name);
        }
    }
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
