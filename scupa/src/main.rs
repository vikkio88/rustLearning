mod mazzu;
use mazzu::{Deck, Hand, Stash};

fn main() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();
    let mut discard_pile = Stash::new();

    deck.shuffle();
    let cards = deck.draw_many(3);
    if let Some(mut cards) = cards {
        hand.add_many(&mut cards)
    }

    println!("Hand:");
    for _ in 0..hand.len() {
        match hand.cards.pop() {
            Some(card) => {
                println!("{}", card);
                discard_pile.add(card);
                println!("cards left in hand: {}", hand.len());
            }
            None => println!("No more cards"),
        }
    }

    println!("cards left: {}", deck.len());
}
