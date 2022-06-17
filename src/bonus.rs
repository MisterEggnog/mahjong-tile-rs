use crate::Tile;

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

impl From<Flowers> for char {
    fn from(tile: Flowers) -> Self {
        match tile {
            Flowers::Plum => '🀢',
            Flowers::Orchid => '🀣',
            Flowers::Chrysanthemum => '🀥',
            Flowers::Bamboo => '🀤',
        }
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

impl From<Seasons> for char {
    fn from(tile: Seasons) -> Self {
        match tile {
            Seasons::Spring => '🀦',
            Seasons::Summer => '🀧',
            Seasons::Autumn => '🀨',
            Seasons::Winter => '🀩',
        }
    }
}

#[test]
fn verify_flowers_amount() {
    let flowers = Flowers::members();
    assert_eq!(4, flowers.count());
}

#[test]
fn verify_flowers_to_char() {
    use std::collections::HashSet;
    let flowers_uniq = Flowers::members()
        .map(|t| match t {
            Tile::Bonus(Bonus::Flowers(f)) => f,
            _ => panic!("Impossible value {:?}", t),
        })
        .map(From::from)
        .collect::<HashSet<char>>();
    assert_eq!(flowers_uniq.len(), Flowers::members().count());
}

#[test]
fn verify_season_amount() {
    let seasons = Seasons::members();
    assert_eq!(4, seasons.count());
}

#[test]
fn verify_seasons_to_char() {
    use std::collections::HashSet;
    let seasons_uniq = Seasons::members()
        .map(|t| match t {
            Tile::Bonus(Bonus::Seasons(s)) => s,
            _ => panic!("Impossible value {:?}", t),
        })
        .map(From::from)
        .collect::<HashSet<char>>();
}
