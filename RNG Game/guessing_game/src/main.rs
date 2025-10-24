use std::io;
use std::cmp::Ordering;
use rand::Rng; // Correct way to use rand in modern Rust

fn main() {
    println!("========== Welcome to My Guessing Game! ==========");

    // Generate random number between 0 and 100
    let my_secret: u32 = rand::thread_rng().gen_range(0..=100);
    println!("I have picked a secret number between 0 and 100. Can you guess it?");

    let mut attempts = 0;

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        // Convert input to number
        let guess: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&my_secret) {
            Ordering::Equal => {
                println!("Correct! You guessed it in {} tries!", attempts);
                println!("---------- Game Over ----------");
                break;
            }
            Ordering::Greater => println!("Incorrect! Try a smaller number."),
            Ordering::Less => println!("Incorrect! Try a bigger number."),
        }
    }
}
