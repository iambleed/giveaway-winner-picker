// Essentially just a number picker
// Used to pick winners of my giveaways

// imports
use std::io::{self, Write};
use rand::Rng;

// main method
fn main() {
    let entries: String = get_input();
    let entries: u32 = entries.trim().parse().expect("Failed to parse into number!");

    let winner = pick_winner(entries);
    println!("Winner: {}", winner);
}

// get user input method
fn get_input() -> String {
    let mut input = String::new();

    print!("Number of entries: ");
    io::stdout().flush().unwrap(); // Making sure the prompt is printed before input
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid line!");

    return input;
}

// pick winner method
fn pick_winner(entries: u32) -> u32 {
    let winner = rand::thread_rng().gen_range(1..=entries);
    return winner;
}