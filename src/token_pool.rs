use crate::gem_pool::GemPool;
use crate::game_config::NumberOfPlayers;

#[derive(Debug, Clone)]
pub struct TokenPool {
	gem_pool: GemPool,
	gold_pool: u8,
}

impl TokenPool {
	pub fn full(number_of_players: NumberOfPlayers) -> Self {
		Self {
			gem_pool: GemPool::new(number_of_players.number_of_gems()),
			gold_pool: 5,
		}
	}

	pub fn empty() -> Self {
		Self {
			gem_pool: GemPool::new(0),
			gold_pool: 0,
		}
	}

	pub fn total(&self) -> u8 {
		self.gem_pool.total() + self.gold_pool
	}

	pub fn gold(&self) -> u8 { self.gold_pool }
	pub fn gems(&self) -> &GemPool { &self.gem_pool }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_full() {
		let token_pool = TokenPool::full(NumberOfPlayers::Two);
		assert_eq!(token_pool.gold_pool, 5);
		assert_eq!(token_pool.gem_pool.total(), 20);
		let token_pool = TokenPool::full(NumberOfPlayers::Three);
		assert_eq!(token_pool.gold_pool, 5);
		assert_eq!(token_pool.gem_pool.total(), 25);
		let token_pool = TokenPool::full(NumberOfPlayers::Four);
		assert_eq!(token_pool.gold_pool, 5);
		assert_eq!(token_pool.gem_pool.total(), 35);
	}

	#[test]
	fn test_empty() {
		let token_pool = TokenPool::empty();
		assert_eq!(token_pool.gold_pool, 0);
		assert_eq!(token_pool.gem_pool.total(), 0);
	}

	#[test]
	fn test_total() {
		let token_pool = TokenPool {
			gold_pool: 10,
			gem_pool: (1, 2, 3, 4, 5).into(),
		};
		assert_eq!(token_pool.total(), 25);
	}
}