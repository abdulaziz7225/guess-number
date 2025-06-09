use guess_number::Guess;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please enter your guess : ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = match Guess::new(guess) {
            Ok(num) => num,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        println!("You guessed : {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
