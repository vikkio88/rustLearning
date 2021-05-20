mod mazzu;
use mazzu::{Deck, Hand, Stash};

mod cli;

fn main() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();
    let mut discard_pile = Stash::new();

    let mut choice = 1;

    while choice != 0 {
        cli::print_menu();
        choice = cli::choose();

        match choice {
            1 => cli::print_hand(&hand),
            2 => cli::print_deck_info(&deck),
            3 => {
                println!("Every day I am shufflin");
                deck.shuffle();
            },
            5 => cli::draw(1, &mut deck, &mut hand),
            6 => cli::draw(2, &mut deck, &mut hand),
            0 => continue,
            _ => {}
        }

        cli::wait_enter();
    }

    println!("kthxbye ;)\n\n");
}
