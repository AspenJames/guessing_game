extern crate rand;

use std::io; // allows us to get input from the user
use std::cmp::Ordering; // allows comparison (line 28)
use rand::Rng; // allows random number generation

fn main() {
    println!("Guess the number!");
    // Generate a secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        // init a variable to store the user's guess
        let mut guess = String::new();
        // read from standard input and store in guess
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // handle failure state
        // turn guess from String type to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // expect a number from stdin
            Err(_) => continue, // continue loop with invalid input
        };
        // display guess to user
        println!("You guessed: {}", guess);
        // match on comparison between guess and secret number
        // display feedback to user
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break out of loop when user guesses correctly
            },
        };
    };
}
