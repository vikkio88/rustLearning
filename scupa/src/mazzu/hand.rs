use super::Card;

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn len(&self) ->usize {
        self.cards.len()
    }

    pub fn add_many(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
    }
}
