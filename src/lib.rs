//! All information source from
//! [https://en.wikipedia.org/wiki/Mahjong_tiles](Wikipedia).
#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Suit(Suit),
    Honor(Honor),
    Bonus(Bonus),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    /// Circles
    Circles(i32),
    /// Bamboo
    Bamboo(i32),
    /// Characters
    Characters(i32),
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

fn make_suits() -> impl Iterator<Item = Tile> {
    // Come on Rust, is an array of function pointers of the SAME TYPE that
    // hard for you?
    let suit_types: [&'static dyn Fn(i32) -> Suit; 3] =
        [&Suit::Circles, &Suit::Bamboo, &Suit::Characters];
    let data: Vec<Tile> = suit_types
        .into_iter()
        .flat_map(|p| (1..=9).map(|i| Tile::Suit(p(i))))
        .collect();
    data.into_iter()
}

#[test]
fn verify_suit_amount() {
    let suits = make_suits();
    assert_eq!(
        3 * 9,
        suits.count(),
        "3 Suits & 9 each should result in {} unique tiles",
        3 * 9
    );
}

// Tiles list
// Pin
pub const IIPIN: Suit = Suit::Circles(1);
pub const RYANPIN: Suit = Suit::Circles(2);
