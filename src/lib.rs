//! All information from
//! <https://en.wikipedia.org/wiki/Mahjong_tiles>.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod bonus;
mod honor;
mod suit;
mod utility;

pub use bonus::*;
pub use honor::*;
pub use suit::*;
use utility::loop_iterator_with;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Suit(Suit),
    Honor(Honor),
    Bonus(Bonus),
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
fn standard_set_amount() {
    let set = standard_set();
    assert_eq!(144, set.count());
}

use bounded_integer::BoundedU8;

// Tiles list
// Pin
pub const IIPIN: Suit = Suit::Circles(match BoundedU8::new(1) {
    Some(a) => a,
    None => unreachable!(),
});
pub const RYANPIN: Suit = Suit::Circles(match BoundedU8::new(2) {
    Some(a) => a,
    None => unreachable!(),
});
// unwrap is not stable for const fn
