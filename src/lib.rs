#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Suit(category::Suit),
    Honor(category::Honor),
    Bonus(category::Bonus),
}

pub mod category {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Suit {
        /// Circles
        Pin(i32),
        /// Bamboo
        So(i32),
        /// Characters
        Man(i32),
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Bonus {
        Seasons(Seasons),
        Flowers(Flowers),
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Honor {
        Winds(Winds),
        Dragons(Dragons),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Winds {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dragons {
    Red,
    Green,
    White,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Seasons {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flowers {
    Plum,
    Orchid,
    Chrysanthemum,
    Bamboo,
}

// Tiles list
// Pin
pub const IIPIN: category::Suit = category::Suit::Pin(1);
pub const RYANPIN: category::Suit = category::Suit::Pin(2);
