//! All information from
//! <https://en.wikipedia.org/wiki/Mahjong_tiles>.
#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Suit(Suit),
    Honor(Honor),
    Bonus(Bonus),
}

type BdU8<const A: u8, const B: u8> = bounded_integer::BoundedU8<A, B>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    /// Circles
    Circles(BdU8<1, 9>),
    /// Bamboo
    Bamboo(BdU8<1, 9>),
    /// Characters
    Characters(BdU8<1, 9>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Honor {
    Winds(Winds),
    Dragons(Dragons),
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
pub enum Bonus {
    Seasons(Seasons),
    Flowers(Flowers),
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
pub const IIPIN: Suit = Suit::Circles(match BdU8::new(1) {
    Some(a) => a,
    None => unreachable!(),
});
pub const RYANPIN: Suit = Suit::Circles(match BdU8::new(2) {
    Some(a) => a,
    None => unreachable!(),
});
// unwrap is not stable for const fn
