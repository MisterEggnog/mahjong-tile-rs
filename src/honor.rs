use crate::Tile;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Honor {
    Winds(Winds),
    Dragons(Dragons),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Winds {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dragons {
    Red,
    Green,
    White,
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
