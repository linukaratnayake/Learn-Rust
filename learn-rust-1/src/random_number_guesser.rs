use rand::Rng;
use std::io; // This is a trait that defines the behavior of random number generators.

use std::cmp::Ordering; // An enum type with variants Less, Greater, Equal.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        // println!("The secret number is {}", secret_number);

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // ignore non-numbers
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // this returns a variant of Ordering
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
