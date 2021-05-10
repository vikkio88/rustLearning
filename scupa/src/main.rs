mod deck;
use deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let hand = deck.draw_many(3);
    println!("Hand:");
    match hand {
        Some(ref hand) => {
            for card in hand {
                println!("\t{}", card);
            }
        }
        None => println!("No cards drawn"),
    }
    let first = &hand.unwrap()[1];
    println!("{}", first.value);

    println!("cards left: {}", deck.left());
}
