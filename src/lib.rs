//! All information from
//! <https://en.wikipedia.org/wiki/Mahjong_tiles>.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod bonus;
mod honor;
mod utility;

use bonus::*;
use honor::*;
use utility::loop_iterator_with;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Suit(Suit),
    Honor(Honor),
    Bonus(Bonus),
}

type SuitNum = bounded_integer::BoundedU8<1, 9>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    Circles(SuitNum),
    Bamboo(SuitNum),
    Characters(SuitNum),
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

/// General set of Mahjong tiles
///
/// Get the common mahjong set.
/// This is 4 of each suit & the honor tiles, plus 1 set of the bonus tiles.
///
/// |Group | Num|
/// |------|--|
/// |Dots  |36|
/// |Bamboo|36|
/// |Characters|36|
/// |Winds |16|
/// |Dragons|12|
/// |Flowers|4|
/// |Seasons|4|
/// |Total|144|
///
/// [^note]: <https://en.wikipedia.org/wiki/Mahjong#Old_Hong_Kong_Mahjong_rules>
pub fn standard_set() -> impl Iterator<Item = Tile> {
    loop_iterator_with(Suit::members, 4)
        .chain(loop_iterator_with(Dragons::members, 4))
        .chain(loop_iterator_with(Winds::members, 4))
        .chain(Flowers::members())
        .chain(Seasons::members())
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
fn standard_set_amount() {
    let set = standard_set();
    assert_eq!(144, set.count());
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
