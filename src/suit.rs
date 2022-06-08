use crate::Tile;

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

impl From<Suit> for char {
    fn from(tile: Suit) -> Self {
        match tile {
            Suit::Characters(n) => match n.get() {
                1 => '🀇',
                2 => '🀈',
                3 => '🀉',
                4 => '🀊',
                5 => '🀋',
                6 => '🀌',
                7 => '🀍',
                8 => '🀎',
                9 => '🀏',
                _ => panic!("{:?}", tile),
            },
            _ => panic!("{:?}", tile),
        }
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
fn verify_unique_char_cast() {
    use std::collections::HashSet;
    let set: HashSet<char> = Suit::members()
        .map(|t| match t {
            Tile::Suit(s) => s,
            _ => unreachable!(),
        })
        .map(|a| a.into())
        .collect();
    assert_eq!(set.len(), Suit::members().count());
}
