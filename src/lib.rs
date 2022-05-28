#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Suit(Suit),
    Honor(Honor),
}

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
pub enum Honor {
    Winds(Winds),
    Dragons(&'static str),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Winds {
    East,
    South,
    West,
    North,
}

// Tiles list
// Pin
pub const IIPIN: Suit = Suit::Pin(1);
pub const RYANPIN: Suit = Suit::Pin(2);
