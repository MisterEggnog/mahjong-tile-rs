use mahjong::*;
use std::iter::once;

#[test]
fn numerical_unicode_equality() {
    let mahjong_chars = (0x1F000..=0x1F02B_u32)
        .map(|c| {
            format!(
                "{}",
                char::from_u32(c).expect("Supposed to be part of unicode SMP")
            )
        })
        .map(|c| {
            if c == "🀄" {
                "🀄︎".to_string()
            } else {
                c
            }
        });
    let tiles = Suit::members()
        .chain(Dragons::members())
        .chain(Winds::members())
        .chain(Seasons::members())
        .chain(Flowers::members())
        .chain(once(Tile::Special(Special::Joker)))
        .chain(once(Tile::Special(Special::Black)));
    let mut tiles: Vec<(&'static str, Tile)> = tiles
        .map(|c| {
            (
                <&'static str>::try_from(c).expect("to str not implemented for this type."),
                c,
            )
        })
        .collect();
    tiles.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    mahjong_chars.zip(tiles).all(|(a, (b, c))| {
        assert_eq!(a, b, "Casting to char for {:?} failed.", c);
        true
    });
}
