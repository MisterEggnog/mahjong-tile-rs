
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
// Pin
pub const iipin: MahjongTile = MahjongTile { suit: Suit::Pin, number: 1 };
pub const ryanpin: MahjongTile = MahjongTile { suit: Suit::Pin, number: 2 };
