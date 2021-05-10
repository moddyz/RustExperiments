extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io; // For standard input.

fn main() {
    println!("Guess the number!");

    // Generate a random number from a thread-local random generator, seeded by the OS.
    // The output range is inclusive of the lower bounds, and exclusive of the upper.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Create a mutable variable binding, from a string.

        io::stdin()
            .read_line(&mut guess) // Pass in guess as a _mutable_ reference.
            .ok() // Discard any _potential_ error info.
            .expect("Failed to read line."); // Panic with a message, if the Result from stdin was non-success.

        println!("You guessed: {}", guess); // {} for printing formatted arguments.

        // Convert the user input into a unsigned 32bit integer.
        // The new numerical guess variable shadows the previous string value of guess.
        let guess: u32 = match guess
            .trim() // Strip leading or terminating whitespace.
            .parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Match expression for the program to behave different depending on
        // comparison results.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
