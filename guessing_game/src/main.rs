use rand::Rng;
use std::cmp::Ordering;
use std::io; // For standard input.

fn main() {
    println!("Guess the number!");

    // Generate a random number from a thread-local random generator, seeded by the OS.  
    // The output range is inclusive of the lower bounds, and exclusive of the upper.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}.", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // Create a mutable variable binding, from a string.

    io::stdin()
        .read_line(&mut guess) // Pass in guess as a _mutable_ reference.
        .ok() // Discard any _potential_ error info.
        .expect("Failed to read line."); // Panic with a message, if the Result from stdin was non-success.

    println!("You guessed: {}", guess); // {} for printing formatted arguments.
   
    // Convert the user input into a unsigned 32bit integer.
    // The new numerical guess variable shadows the previous string value of guess.
    let guess: u32 = guess.trim() // Strip leading or terminating whitespace.
        .parse() // Convert the string into the type annotated after the : in the above line.
        .expect("Please type a number!"); // Handle parsing string -> numeric conversion error.
    
    // Match expression for the program to behave different depending on
    // comparison results.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
