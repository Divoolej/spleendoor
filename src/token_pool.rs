use crate::gem_pool::GemPool;
use crate::game_config::NumberOfPlayers;

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
}
