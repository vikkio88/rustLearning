use super::Card;

pub struct Hand {
    max: usize,
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::new(),
            max: 0,
        }
    }

    pub fn set_max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn space(&self) -> usize {
        if self.max == 0 {
            return 0;
        }
        self.max - self.len()
    }

    pub fn can_add(&self) -> bool {
        self.max == 0 || (self.max > 0 && self.len() == self.max)
    }

    pub fn add_many(&mut self, cards: &mut Vec<Card>) -> bool {
        if !self.can_add() {
            return false;
        }

        self.cards.append(cards);
        true
    }
}
