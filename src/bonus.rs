use crate::Tile;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Bonus {
    Seasons(Seasons),
    Flowers(Flowers),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Seasons {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Flowers {
    Plum,
    Orchid,
    Chrysanthemum,
    Bamboo,
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

impl Seasons {
    pub fn members() -> impl Iterator<Item = Tile> {
        [
            Seasons::Spring,
            Seasons::Summer,
            Seasons::Autumn,
            Seasons::Winter,
        ]
        .into_iter()
        .map(|s| Tile::Bonus(Bonus::Seasons(s)))
    }
}

#[test]
fn verify_flowers_amount() {
    let flowers = Flowers::members();
    assert_eq!(4, flowers.count());
}

#[test]
fn verify_season_amount() {
    let seasons = Seasons::members();
    assert_eq!(4, seasons.count());
}
