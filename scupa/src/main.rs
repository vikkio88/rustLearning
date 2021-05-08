mod deck;
use deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    for _ in 0..41 {
        println!("Cards left: {}", deck.left());
        match deck.draw() {
            Some(card) => println!("{}", card),
            None => println!("No more cards!"),
        }
    }
}
