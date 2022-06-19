use mahjong::*;
use std::iter::once;

#[test]
fn numerical_unicode_equality() {
    let mahjong_chars = (0x1F000..=0x1F02B_u32)
        .map(|c| char::from_u32(c).expect("Supposed to be part of unicode SMP"));
    let tiles = Suit::members()
        .chain(Dragons::members())
        .chain(Winds::members())
        .chain(Seasons::members())
        .chain(Flowers::members())
        .chain(once(Tile::Special(Special::Joker)))
        .chain(once(Tile::Special(Special::Black)));

    let mut tiles: Vec<(char, Tile)> = tiles
        .map(|c| (char::try_from(c).expect("These should not fail"), c))
        .collect();
    tiles.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    mahjong_chars.zip(tiles).all(|(a, (b, c))| {
        assert_eq!(a, b, "Casting to char for {:?} failed.", c);
        true
    });
}
