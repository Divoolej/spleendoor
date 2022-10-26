use crate::game_config::NumberOfPlayers;
use crate::gem_pool::GemPool;

pub type TokenPoolTuple = (u8, u8, u8, u8, u8, u8);

#[derive(Debug, Clone, Copy)]
pub struct TokenPool {
	pub gems: GemPool,
	pub gold: u8,
}

impl TokenPool {
	pub fn full(number_of_players: NumberOfPlayers) -> Self {
		Self {
			gems: GemPool::new(number_of_players.number_of_gems()),
			gold: 5,
		}
	}

	pub fn empty() -> Self {
		Self {
			gems: GemPool::new(0),
			gold: 0,
		}
	}

	pub fn total(&self) -> u8 {
		self.gems.total() + self.gold
	}
}

impl From<TokenPoolTuple> for TokenPool {
	fn from((diamonds, sapphires, emeralds, rubies, onyxes, gold): TokenPoolTuple) -> Self {
		Self {
			gems: (diamonds, sapphires, emeralds, rubies, onyxes).into(),
			gold: gold,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_full() {
		let token_pool = TokenPool::full(NumberOfPlayers::Two);
		assert_eq!(token_pool.gold, 5);
		assert_eq!(token_pool.gems.total(), 20);
		let token_pool = TokenPool::full(NumberOfPlayers::Three);
		assert_eq!(token_pool.gold, 5);
		assert_eq!(token_pool.gems.total(), 25);
		let token_pool = TokenPool::full(NumberOfPlayers::Four);
		assert_eq!(token_pool.gold, 5);
		assert_eq!(token_pool.gems.total(), 35);
	}

	#[test]
	fn test_empty() {
		let token_pool = TokenPool::empty();
		assert_eq!(token_pool.gold, 0);
		assert_eq!(token_pool.gems.total(), 0);
	}

	#[test]
	fn test_total() {
		let token_pool = TokenPool {
			gold: 10,
			gems: (1, 2, 3, 4, 5).into(),
		};
		assert_eq!(token_pool.total(), 25);
	}
}
