use crate::card::Card;
use crate::cards::Cards;
use crate::gem_pool::GemPool;
use crate::player::{Player, PlayerIndex};
use crate::token_pool::TokenPool;
use crate::aristocrats::Aristocrats;
use crate::game_config::{GameConfig, NumberOfPlayers};


pub struct Game {
	turn: u8,
	cards: Cards,
	players: Vec<Player>,
	token_pool: TokenPool,
	aristocrats: Aristocrats,
	current_player: PlayerIndex,
	starting_player: PlayerIndex,
	number_of_players: NumberOfPlayers,
}

impl Game {
	pub fn from_game_config(config: &GameConfig) -> Self {
		let starting_player: PlayerIndex = rand::random::<usize>() % config.number_of_players.count();

		Self {
			turn: 1,
			cards: Cards::deal(),
			current_player: starting_player,
			number_of_players: config.number_of_players,
			token_pool: TokenPool::full(config.number_of_players),
			aristocrats: Aristocrats::deal(config.number_of_players),
			players: vec![Player::new(); config.number_of_players.count()],
			starting_player,
		}
	}

	pub fn turn(&self) -> u8 { self.turn }
	pub fn players(&self) -> &Vec<Player> { &self.players }
	pub fn gold_pool(&self) -> u8 { self.token_pool.gold() }
	pub fn gem_pool(&self) -> &GemPool { self.token_pool.gems() }
	pub fn aristocrat_pool(&self) -> &Aristocrats { &self.aristocrats }
	pub fn current_player(&self) -> PlayerIndex { self.current_player }
	pub fn tier_1_card_pool(&self) -> &Vec<Card> { self.cards.tier_1() }
	pub fn tier_2_card_pool(&self) -> &Vec<Card> { self.cards.tier_2() }
	pub fn tier_3_card_pool(&self) -> &Vec<Card> { self.cards.tier_3() }
	pub fn starting_player(&self) -> PlayerIndex { self.starting_player }
	pub fn number_of_players(&self) -> usize { self.number_of_players.count() }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_game_config() {
		let game_config = GameConfig { number_of_players: NumberOfPlayers::Three };
		let game = Game::from_game_config(&game_config);

		assert_eq!(game.turn, 1);
		assert!(!game.cards.tier_1().is_empty());
		assert!(!game.cards.tier_2().is_empty());
		assert!(!game.cards.tier_3().is_empty());
		assert!(game.starting_player < game.players.len());
		assert_eq!(game.current_player, game.starting_player);
		assert_eq!(game.number_of_players, NumberOfPlayers::Three);
		assert_eq!(game.players.len(), NumberOfPlayers::Three.count());
		assert_ne!(game.token_pool.total(), 0);
		assert_ne!(game.aristocrats.len(), 0);
	}
}