use crate::game_config::{GameConfig, NumberOfPlayers};
use crate::token_pool::TokenPool;
use crate::cards::Cards;

pub struct Game {
	number_of_players: NumberOfPlayers,
	token_pool: TokenPool,
	cards: Cards,
}

impl Game {
	pub fn from_game_config(config: &GameConfig) -> Self {
		Self {
			number_of_players: config.number_of_players,
			token_pool: TokenPool::full(config.number_of_players),
			cards: Cards::deal(),
		}
	}
}
