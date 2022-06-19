use crate::Tile;
use serde::{Deserialize, Serialize};

type SuitNum = bounded_integer::BoundedU8<1, 9>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[derive(Serialize, Deserialize)]
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
