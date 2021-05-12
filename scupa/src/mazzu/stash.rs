use super::Card;

pub struct Stash {
    cards: Vec<Card>,
}

impl Stash {
    pub fn new() -> Stash {
        let stash: Vec<Card> = Vec::new();

        Stash { cards: stash }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}