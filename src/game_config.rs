#[derive(Clone, Copy)]
pub enum NumberOfPlayers { Two, Three, Four }

impl NumberOfPlayers {
	pub fn number_of_gems(&self) -> u8 {
		match self {
			Self::Two => 4,
			Self::Three => 5,
			Self::Four => 7
		}
	}

	pub fn number_of_aristocrats(&self) -> u8 {
		match self {
			Self::Two => 3,
			Self::Three => 4,
			Self::Four => 5,
		}
	}
}

pub struct GameConfig {
	pub number_of_players: NumberOfPlayers,
}
