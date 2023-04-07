use std::{cmp::Ordering, io};

use rand::Rng;

const MAX_ATTEMPTS: i8 = 3;

fn main() {
    println!("I am thinking of a number...");
    let number: u32 = rand::thread_rng().gen_range(0..=100);
    let mut reamining_guesses = MAX_ATTEMPTS;
    println!("You have {MAX_ATTEMPTS} guesses.");
    loop {
        println!("Guesses left {reamining_guesses} / {MAX_ATTEMPTS}");
        println!("Guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Hey! That is not a number!");
                continue;
            }
        };

        println!("You guessed {guess}");
        reamining_guesses -= 1;

        if check_result(&guess, &number) {
            break;
        }

        if reamining_guesses <= 0 {
            println!("You have no guesses left.\n the number was {number}\n GAME OVER!");
            break;
        }
    }
}

fn check_result(guess: &u32, number: &u32) -> bool {
    match guess.cmp(number) {
        Ordering::Equal => {
            println!("You guessed it right! The number was {number}");
            true
        }
        Ordering::Greater => {
            println!("Your guess is too high!");
            false
        }
        Ordering::Less => {
            println!("Your guess is too low!");
            false
        }
    }
}
