#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

	pub fn count(&self) -> usize {
		match self {
			Self::Two => 2,
			Self::Three => 3,
			Self::Four => 4,
		}
	}
}

pub struct GameConfig {
	pub number_of_players: NumberOfPlayers,
	pub seed: Option<String>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_number_of_gems() {
		assert_eq!(NumberOfPlayers::Two.number_of_gems(), 4);
		assert_eq!(NumberOfPlayers::Three.number_of_gems(), 5);
		assert_eq!(NumberOfPlayers::Four.number_of_gems(), 7);
	}

	#[test]
	fn test_number_of_aristocrats() {
		assert_eq!(NumberOfPlayers::Two.number_of_aristocrats(), 3);
		assert_eq!(NumberOfPlayers::Three.number_of_aristocrats(), 4);
		assert_eq!(NumberOfPlayers::Four.number_of_aristocrats(), 5);
	}

	#[test]
	fn test_player_count() {
		assert_eq!(NumberOfPlayers::Two.count(), 2);
		assert_eq!(NumberOfPlayers::Three.count(), 3);
		assert_eq!(NumberOfPlayers::Four.count(), 4);
	}
}
