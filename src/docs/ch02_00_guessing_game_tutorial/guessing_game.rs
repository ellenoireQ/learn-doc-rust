/// Rusian Roulette game.
/// WIKI: https://en.wikipedia.org/wiki/Russian_roulette
/// Same concept with guessing game from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
/// but add some modification
use std::io;

use rand::Rng;

pub fn guessing_game() {
    // random bullet generator
    let bullet = rand::rng().random_range(1..=10);

    println!("Russian Roulette");
    println!("Guess the number! 1-10");

    println!("Please input your guess.");

    // see: https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        // First we need to parsing to int
        let number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Failed to parse number: {}", e);
                return;
            }
        };
        if number == bullet {
            println!("Yahh you been killed by bullet");
            return;
        }
        println!("Nope, try again!");
    }
}
