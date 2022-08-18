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

	pub fn gold(&self) -> u8 { self.gold_pool }

	pub fn gems(&self) -> &GemPool { &self.gem_pool }
}
