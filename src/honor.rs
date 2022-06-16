use crate::Tile;

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

impl From<Winds> for char {
    fn from(tile: Winds) -> Self {
        match tile {
            Winds::East => '🀀',
            Winds::South => '🀁',
            Winds::West => '🀂',
            Winds::North => '🀃',
        }
    }
}

impl From<Dragons> for char {
    fn from(tile: Dragons) -> Self {
        match tile {
            Dragons::Red => '🀄',
            Dragons::Green => '🀅',
            Dragons::White => '🀆',
        }
    }
}

impl From<Honor> for char {
    fn from(tile: Honor) -> Self {
        match tile {
            Honor::Winds(w) => w.into(),
            Honor::Dragons(d) => d.into(),
        }
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

#[test]
fn verify_winds_to_char() {
    use std::collections::HashSet;
    let winds_char_uniq = Winds::members()
        .map(|t| match t {
            Tile::Honor(Honor::Winds(d)) => d,
            _ => panic!("Impossible value {:?}", t),
        })
        .map(From::from)
        .collect::<HashSet<char>>();
    assert_eq!(winds_char_uniq.len(), Winds::members().count());
}

#[test]
fn verify_dragons_to_char() {
    use std::collections::HashSet;
    let dragons_char_uniq = Dragons::members()
        .map(|t| match t {
            Tile::Honor(Honor::Dragons(d)) => d,
            _ => panic!("Impossible value {:?}", t),
        })
        .map(From::from)
        .collect::<HashSet<char>>();
    assert_eq!(dragons_char_uniq.len(), Dragons::members().count());
}

#[test]
fn verify_honors_to_char() {
    use std::collections::HashSet;
    let honor_char_uniq = Dragons::members()
        .chain(Winds::members())
        .map(|t| match t {
            Tile::Honor(h) => h,
            _ => panic!("Impossible value {:?}", t),
        })
        .map(From::from)
        .collect::<HashSet<char>>();
    assert_eq!(
        honor_char_uniq.len(),
        Dragons::members().chain(Winds::members()).count()
    );
}
