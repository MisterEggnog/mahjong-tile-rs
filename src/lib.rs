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

type SuitNum = bounded_integer::BoundedU8<1, 9>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    /// Circles
    Circles(SuitNum),
    /// Bamboo
    Bamboo(SuitNum),
    /// Characters
    Characters(SuitNum),
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

impl Suit {
    pub fn members() -> impl Iterator<Item = Tile> {
        // Come on Rust, is an array of function pointers of the SAME TYPE that
        // hard for you?
        let suit_types: [&'static dyn Fn(SuitNum) -> Suit; 3] =
            [&Suit::Circles, &Suit::Bamboo, &Suit::Characters];
        suit_types.into_iter().flat_map(|p| {
            (1..=9)
                .map(|i| SuitNum::new(i).expect("Input in range 1..=9"))
                .map(|i| Tile::Suit(p(i)))
        })
    }
}

impl Dragons {
    pub fn members() -> impl Iterator<Item = Tile> {
        [Dragons::Red, Dragons::Green, Dragons::White]
            .into_iter()
            .map(|d| Tile::Honor(Honor::Dragons(d)))
    }
}

impl Winds {
    pub fn members() -> impl Iterator<Item = Tile> {
        [Winds::East, Winds::South, Winds::West, Winds::North]
            .into_iter()
            .map(|w| Tile::Honor(Honor::Winds(w)))
    }
}

impl Flowers {
    pub fn members() -> impl Iterator<Item = Tile> {
        [
            Flowers::Plum,
            Flowers::Orchid,
            Flowers::Chrysanthemum,
            Flowers::Bamboo,
        ]
        .into_iter()
        .map(|f| Tile::Bonus(Bonus::Flowers(f)))
    }
}

#[test]
fn verify_suit_amount() {
    let suits = Suit::members();
    assert_eq!(
        3 * 9,
        suits.count(),
        "3 Suits & 9 each should result in {} unique tiles",
        3 * 9
    );
}

#[test]
fn verify_dragon_amount() {
    let dragons = Dragons::members();
    assert_eq!(3, dragons.count());
}

#[test]
fn verify_winds_amount() {
    let winds = Winds::members();
    assert_eq!(4, winds.count());
}

#[test]
fn verify_flowers_amount() {
    let flowers = Flowers::members();
    assert_eq!(4, flowers.count());
}

// Tiles list
// Pin
pub const IIPIN: Suit = Suit::Circles(match SuitNum::new(1) {
    Some(a) => a,
    None => unreachable!(),
});
pub const RYANPIN: Suit = Suit::Circles(match SuitNum::new(2) {
    Some(a) => a,
    None => unreachable!(),
});
// unwrap is not stable for const fn
