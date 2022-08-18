use crate::aristocrat::Aristocrat;
use crate::aristocrats::Aristocrats;
use crate::game_config::{GameConfig, NumberOfPlayers};
use crate::token_pool::TokenPool;
use crate::card::Card;
use crate::cards::Cards;
use crate::gem_pool::GemPool;

pub struct Game {
	number_of_players: NumberOfPlayers,
	token_pool: TokenPool,
	aristocrats: Aristocrats,
	cards: Cards,
}

impl Game {
	pub fn from_game_config(config: &GameConfig) -> Self {
		Self {
			number_of_players: config.number_of_players,
			token_pool: TokenPool::full(config.number_of_players),
			aristocrats: Aristocrats::deal(config.number_of_players),
			cards: Cards::deal(),
		}
	}

	pub fn gold_left(&self) -> u8 {
		self.token_pool.gold()
	}

	pub fn gems_left(&self) -> &GemPool {
		self.token_pool.gems()
	}

	pub fn aristocrats_left(&self) -> &Aristocrats {
		&self.aristocrats
	}

	pub fn number_of_players(&self) -> u8 {
		self.number_of_players.into()
	}

	pub fn tier_1_cards_left(&self) -> &Vec<Card> {
		self.cards.tier_1()
	}

	pub fn tier_2_cards_left(&self) -> &Vec<Card> {
		self.cards.tier_2()
	}

	pub fn tier_3_cards_left(&self) -> &Vec<Card> {
		self.cards.tier_3()
	}
}