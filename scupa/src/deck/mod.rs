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

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(40);
        let suits = vec![Suit::Bastoni, Suit::Coppe, Suit::Denari, Suit::Mazze];
        for suit in suits {
            for i in 0..10 {
                cards.push(Card { value: i + 1, suit });
            }
        }
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_many(&mut self, cards: usize) -> Option<Vec<Card>> {
        if cards < 1 || self.left() < cards {
            return None;
        }

        let mut hand: Vec<Card> = Vec::with_capacity(cards);

        for _ in 0..cards {
            if let Some(card) = self.draw() {
                hand.push(card)
            }
        }

        Some(hand)
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn left(&self) -> usize {
        self.cards.len()
    }
}
