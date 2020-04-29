use std::io; // For standard input.

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // Create a mutable variable binding, from a string.

    io::stdin()
        .read_line(&mut guess) // Pass in guess as a _mutable_ reference.
        .ok() // Discard any _potential_ error info.
        .expect("Failed to read line."); // Panic with a message, if the Result from stdin was non-success.

    println!("You guessed: {}", guess); // {} for printing formatted arguments.
}
