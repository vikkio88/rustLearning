mod deck;
use deck::{Deck, Stash};

fn main() {
    let mut deck = Deck::new();
    let mut discard_pile = Stash::new();

    deck.shuffle();
    let hand = deck.draw_many(3);
    println!("Hand:");
    match hand {
        Some(mut hand) => {
            for _ in 0..hand.len() {
                match hand.pop() {
                    Some(card) => {
                        println!("{}", card);
                        discard_pile.add(card);
                        println!("cards left in hand: {}", hand.len());
                    }
                    None => println!("No more cards"),
                };
            }
        }
        None => println!("No cards drawn"),
    }

    println!("cards left: {}", deck.left());
}
