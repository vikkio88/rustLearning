
use super::Card;
use super::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;

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
                hand.push(card);
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
