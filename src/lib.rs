#![allow(dead_code)]
#![allow(unused_imports)]

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
pub const IIPIN: SuitTile = SuitTile {
    suit: Suit::Pin,
    number: 1,
};
pub const RYANPIN: SuitTile = SuitTile {
    suit: Suit::Pin,
    number: 2,
};
