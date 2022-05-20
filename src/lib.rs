
use count_macro::count;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MahjongTile {
	suit: Suit,
	number: i32,
}

impl MahjongTile {
	pub fn get_suit(&self) -> Suit {
		self.suit
	}

	pub fn get_number(&self) -> i32 {
		self.number
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
	/// Circles
	Pin,
	/// Bamboo
	So,
	/// Characters
	Man,
	/// Not a true suit, only 4 tiles
	Wind,
	/// Not a true suit, on 3 tiles
	Dragon,
}

// Tiles list
count! {
// Pin
pub const iipin: MahjongTile = MahjongTile { suit: Suit::Pin, number: _int_pin_ };
pub const ryanpin: MahjongTile = MahjongTile { suit: Suit::Pin, number: _int_pin_ };
}
