//! Implements mahjong tile type in rust.
//! This is likely over engineered & will probably change in the future.
//! All the information for this came from Wikipedia.
//!
//! #### Impl From<Tile> for char
//! Casting [Tile] to char is currently possible, though it's pretty funky.
//! One tile is an emoji but the rest are not.
//! Plus while right now all Tiles can be cast to chars, this will change in the future.
//! Furthermore some types may broadly map to one char, e.g. [Special::Joker].
//! In the future this may become a feature.
//!
//! #### Deriving Serde
//! To enable deriving serde, simply set the serde feature.
//!
//! ### Sources:
//! [Mahjong Tiles] <br>
//! [Mahjong Unicode Block] <br>
//!
//! [Mahjong Tiles]: https://en.wikipedia.org/wiki/Mahjong_tiles
//! [Mahjong Unicode Block]: https://en.wikipedia.org/wiki/Mahjong_Tiles_(Unicode_block)

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod bonus;
mod honor;
mod suit;
mod utility;

pub use bonus::*;
pub use honor::*;
pub use suit::*;
use utility::loop_iterator_with;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum Tile {
    Suit(Suit),
    Honor(Honor),
    Bonus(Bonus),
    Special(Special),
}

impl TryFrom<Tile> for char {
    type Error = std::convert::Infallible;
    // Removed TileCastingError on 2022/06/30.
    // Currently it's a clippy warning to have unreachable branches, even on
    // a type marked non_exhaustive.
    // So revert this when the Tile type expands.

    fn try_from(value: Tile) -> Result<Self, Self::Error> {
        match value {
            Tile::Suit(suit) => Ok(suit.into()),
            Tile::Honor(honor) => Ok(honor.into()),
            Tile::Bonus(bonus) => Ok(bonus.into()),
            Tile::Special(special) => Ok(special.into()),
        }
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
    let four_time_tiles = || {
        Suit::members()
            .chain(Dragons::members())
            .chain(Winds::members())
    };
    loop_iterator_with(four_time_tiles, 4)
        .chain(Flowers::members())
        .chain(Seasons::members())
}

#[test]
fn standard_set_amount() {
    let set = standard_set();
    assert_eq!(144, set.count());
}

/// Unicode codespace defined tiles
///
/// These tiles are defined in the Unicode codespace.
/// Therefore I'm including them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Special {
    /// Probably a bigger deal in American Mahjong.
    Joker,
    /// I don't know what this tile is.
    Black,
}

impl From<Special> for char {
    fn from(tile: Special) -> Self {
        match tile {
            Special::Joker => '????',
            Special::Black => '????',
        }
    }
}
