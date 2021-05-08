use rand::prelude::*;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut attemps: u8 = 3;
    let number: u8 = (random::<f32>() * 10.) as u8 % 10 + 1;
    println!(
        "Yo I thought of a number between 1-10, you have {} chances to guess",
        attemps
    );
    let mut choice: u8;
    while attemps > 0 {
        choice = choose();
        if choice == number {
            println!("Yo got it it was {}", choice);
            break;
        } else {
            attemps -= 1;
            println!(
                "nope, it wasnt {}, my number is {} (attempts left: {})",
                choice,
                if choice > number { "lower" } else { "higher" },
                attemps
            );
        }
    }

    if attemps == 0 {
        println!("Ah too bad mate, it was {}", number);
    }
}

fn choose() -> u8 {
    print!("Choose > ");
    let _ = stdout().flush();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<u8>().unwrap_or(0)
}
