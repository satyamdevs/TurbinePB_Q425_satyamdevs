use std::io::{self, Write};
use std::{thread::sleep, time::{Duration, Instant}};

fn main() {
    let start = Instant::now();
    let mut counter: u32 = 0;

    loop {
        counter += 1;
        println!(" Counter: {}", counter);
        sleep(Duration::from_millis(100));

        if counter % 10 == 0 {
            print!("Do you want the counter to stop? (y/n): ");
            io::stdout().flush().unwrap(); 

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.eq_ignore_ascii_case("y") {
                break;
            }
        }

        if counter >= 1_000_000 {
            println!("Reached the limit of 1,000,000. Stopping...");
            break;
        }
    }

    let elapsed = start.elapsed();
    println!("Program executed in: {:.2?} seconds", elapsed);
}
