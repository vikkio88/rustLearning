use std::io::{stdin, stdout, Write};

pub fn print_menu() {
    println!(
        r#"
    1) Print Hand
    2) Cards Left
    3) Print Discard Pile (not implemented)

    4) Draw 1
    5) Draw 2
    

    0) quit

    "#
    )
}

pub fn print_hand(hand: &super::mazzu::Hand) {
    println!("Hand: ");
    for c in &hand.cards {
        println!("{}", c);
    }
}

pub fn print_deck_info(deck: &super::mazzu::Deck) {
    println!("Deck: Cards Left: {}", deck.len());
}

pub fn choose() -> u8 {
    print!(" > ");
    let _ = stdout().flush();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<u8>().unwrap_or(0)
}

pub fn wait_enter() {
    println!("\n\n...[Enter]...");
    let _ = stdout().flush();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}

pub fn draw(cards_num: usize, deck: &mut super::mazzu::Deck, hand: &mut super::mazzu::Hand) {
    let cards = deck.draw_many(cards_num);
    if let Some(mut cards) = cards {
        println!(
            "Drawn {} card{}",
            cards.len(),
            if cards.len() > 1 { "s" } else { "" }
        );
        for card in &cards {
            println!("{}", card);
        }
        hand.add_many(&mut cards);
    } else {
        println!("Couldnt draw card");
    }
}
