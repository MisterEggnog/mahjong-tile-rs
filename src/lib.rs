#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SuitTile {
    suit: Suit,
    number: i32,
}

impl SuitTile {
    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    /// Circles
    Pin,
    /// Bamboo
    So,
    /// Characters
    Man,
}

// Tiles list
// Pin
pub const iipin: SuitTile = SuitTile {
    suit: Suit::Pin,
    number: 1,
};
pub const ryanpin: SuitTile = SuitTile {
    suit: Suit::Pin,
    number: 2,
};
