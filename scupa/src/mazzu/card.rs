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
        write!(fmt, "{} {}", &self.value, self.suit)
    }
}
