use crate::gem_pool::GemPool;
use crate::game_config::NumberOfPlayers;

pub struct TokenPool {
	gem_pool: GemPool,
	gold_pool: u8,
}

impl TokenPool {
	pub fn full(number_of_players: NumberOfPlayers) -> Self {
		use NumberOfPlayers::*;

		let gem_count = match number_of_players { Two => 4, Three => 5, Four => 7 };
		let gem_pool = GemPool::from((gem_count, gem_count, gem_count, gem_count, gem_count));

		Self {
			gem_pool,
			gold_pool: 5,
		}
	}
}
