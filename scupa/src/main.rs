mod deck;
use deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let hand = deck.draw_many(3);
    println!("Hand:");
    for card in hand {
        println!("\t{}", card);
    }

    println!("cards left: {}", deck.left());
}
