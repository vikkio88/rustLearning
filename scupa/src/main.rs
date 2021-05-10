mod deck;
use deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let hand = deck.draw_many(3);
    println!("Hand:");
    match hand {
        Some(hand) => {
            for card in hand {
                println!("\t{}", card);
            }
        }
        None => println!("No cards drawn"),
    }

    println!("cards left: {}", deck.left());
}
