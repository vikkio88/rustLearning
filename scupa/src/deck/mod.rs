use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Bastoni,
    Denari,
    Coppe,
    Mazze,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

pub struct Card {
    pub value: u8,
    pub suit: Suit,
}

impl std::fmt::Display for Card {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{} {}", self.value, self.suit)
    }
}

pub struct Deck {
    _deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Vec::with_capacity(40);
        let suits = vec![Suit::Bastoni, Suit::Coppe, Suit::Denari, Suit::Mazze];
        for suit in suits {
            for i in 0..10 {
                deck.push(Card {
                    value: i + 1,
                    suit: suit,
                });
            }
        }
        Deck { _deck: deck }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self._deck.pop()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self._deck.shuffle(&mut rng);
    }

    pub fn left(&self) -> usize {
        self._deck.len()
    }
}
