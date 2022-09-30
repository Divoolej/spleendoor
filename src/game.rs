use log::trace;
use rand::rngs::StdRng;

use crate::action::Action;
use crate::aristocrat::Aristocrat;
use crate::aristocrats::Aristocrats;
use crate::card::Card;
use crate::cards::Cards;
use crate::game_config::{GameConfig, NumberOfPlayers};
use crate::gem::Gem;
use crate::gem_pool::GemPool;
use crate::player::{Player, PlayerIndex};
use crate::token_pool::TokenPool;
use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
	AwaitingAction,
	AwaitingDiscard,
	GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleActionError {
	InvalidGameState,
	NotEnoughGems,
	InvalidCardIndex,
}

#[derive(Debug)]
pub struct BoardState {
	pub turn: u8,
	pub cards: Cards,
	pub players: Vec<Player>,
	pub token_pool: TokenPool,
	pub game_state: GameState,
	pub aristocrats: Vec<Aristocrat>,
	pub current_player_index: PlayerIndex,
}

#[cfg(debug_assertions)]
impl BoardState {
	pub fn pretty_print(&self) {
		println!(
			"T{} P{} <{:?}> D{}|S{}|E{}|R{}|O{}|G{}",
			self.turn,
			self.current_player_index,
			self.game_state,
			self.token_pool.gems().diamonds(),
			self.token_pool.gems().sapphires(),
			self.token_pool.gems().emeralds(),
			self.token_pool.gems().rubies(),
			self.token_pool.gems().onyxes(),
			self.token_pool.gold()
		);
		print!("A: ");
		for a in &self.aristocrats {
			a.pretty_print();
			print!(" ");
		}
		print!("\n");
		self.cards.pretty_print();
		for (i, p) in self.players.iter().enumerate() {
			print!("\nP{} ", i);
			p.gems().pretty_print();
			print!(" ");
			for c in p.cards() {
				match c.gem() {
					Gem::Diamond => print!("D"),
					Gem::Sapphire => print!("S"),
					Gem::Emerald => print!("E"),
					Gem::Ruby => print!("R"),
					Gem::Onyx => print!("O"),
				}
				print!("[{}] ", c.points());
			}
		}
		print!("\n");
	}
}

pub struct Game {
	turn: u8,
	rng: StdRng,
	seed: String,
	cards: Cards,
	players: Vec<Player>,
	token_pool: TokenPool,
	game_state: GameState,
	aristocrats: Aristocrats,
	current_player_index: PlayerIndex,
	starting_player_index: PlayerIndex,
	number_of_players: NumberOfPlayers,
}

impl Game {
	pub fn from_config(config: GameConfig) -> Self {
		use rand::SeedableRng;

		let starting_player_index: PlayerIndex = rand::random::<usize>() % config.number_of_players.count();
		let seed = config.seed.filter(|s| !s.is_empty()).unwrap_or_else(utils::generate_seed); // TODO: Return error when seed is empty?
		let mut rng = StdRng::from_seed(
			seed.as_bytes().iter().copied().cycle().take(32).collect::<Vec<u8>>().try_into().unwrap(), // Safe because we ensure that the string is not empty
		);

		trace!(
			"Starting a new {}-player game with seed {}. Starting player: {}",
			config.number_of_players.count(),
			seed,
			starting_player_index
		);

		Self {
			players: vec![Player::new(); config.number_of_players.count()],
			aristocrats: Aristocrats::deal(&mut rng, config.number_of_players),
			token_pool: TokenPool::full(config.number_of_players),
			current_player_index: starting_player_index,
			number_of_players: config.number_of_players,
			game_state: GameState::AwaitingAction,
			cards: Cards::deal(&mut rng),
			starting_player_index,
			turn: 1,
			seed,
			rng,
		}
	}

	pub fn handle_action(&mut self, action: Action) -> Result<(), HandleActionError> {
		if !matches!(self.game_state, GameState::AwaitingAction) {
			return Err(HandleActionError::InvalidGameState);
		}

		let player = &mut self.players[self.current_player_index];

		trace!("Player {} |> {:?}", self.current_player_index, action);

		match action {
			Action::TakeTwoGems(gem) => {
				if self.token_pool.gems().count(gem) >= 4 {
					self.token_pool.gems_mut().remove(gem, 2);
					player.add_gem(gem, 2);
				} else {
					return Err(HandleActionError::NotEnoughGems);
				}
				if player.token_count() > 10 {
					self.game_state = GameState::AwaitingDiscard;
					return Ok(());
				}
			}
			Action::TakeThreeGems(first_gem, second_gem, third_gem) => {
				if self.token_pool.gems().count(first_gem) > 0
					&& self.token_pool.gems().count(second_gem) > 0
					&& self.token_pool.gems().count(third_gem) > 0
				{
					self.token_pool.gems_mut().remove(first_gem, 1);
					self.token_pool.gems_mut().remove(second_gem, 1);
					self.token_pool.gems_mut().remove(third_gem, 1);
					player.add_gem(first_gem, 1);
					player.add_gem(second_gem, 1);
					player.add_gem(third_gem, 1);
				} else {
					return Err(HandleActionError::NotEnoughGems);
				}
				if player.token_count() > 10 {
					self.game_state = GameState::AwaitingDiscard;
					return Ok(());
				}
			}
			Action::BuyCard(tier, index) => {
				let index = index as usize;
				if index > 3 || self.cards.tier(tier).len() < index {
					return Err(HandleActionError::InvalidCardIndex);
				}
				let card = &self.cards.tier(tier)[index];
				if player.can_buy(card) {
					let gold_needed = GemPool::difference(card.cost(), &player.effective_gem_pool());
					player.remove_gold(gold_needed.total());
					let gem_cost = GemPool::difference(card.cost(), &player.cards_gem_pool());
					player.remove_gem(Gem::Diamond, gem_cost.diamonds());
					player.remove_gem(Gem::Sapphire, gem_cost.sapphires());
					player.remove_gem(Gem::Emerald, gem_cost.emeralds());
					player.remove_gem(Gem::Ruby, gem_cost.rubies());
					player.remove_gem(Gem::Onyx, gem_cost.onyxes());
					player.cards_mut().push(self.cards.tier_mut(tier).remove(index));
				}
			}
			Action::BuyReservedCard(_) => todo!(),
			Action::ReserveCard(_, _) => todo!(),
			Action::Pass => todo!(),
		};

		self.turn += 1;
		self.current_player_index = (self.current_player_index + 1) % self.number_of_players.count();
		Ok(())
	}

	pub fn board_state(&self) -> BoardState {
		BoardState {
			turn: self.turn,
			game_state: self.game_state,
			token_pool: self.token_pool,
			cards: self.cards.visible(),
			aristocrats: (*self.aristocrats).clone(),
			players: self.players.clone(),
			current_player_index: self.current_player_index,
		}
	}

	pub fn turn(&self) -> u8 {
		self.turn
	}
	pub fn game_state(&self) -> GameState {
		self.game_state
	}
	pub fn players(&self) -> &Vec<Player> {
		&self.players
	}
	pub fn gold_pool(&self) -> u8 {
		self.token_pool.gold()
	}
	pub fn gem_pool(&self) -> &GemPool {
		self.token_pool.gems()
	}
	pub fn aristocrat_pool(&self) -> &Aristocrats {
		&self.aristocrats
	}
	pub fn current_player_index(&self) -> PlayerIndex {
		self.current_player_index
	}
	pub fn tier_1_card_pool(&self) -> &Vec<Card> {
		self.cards.tier_1()
	}
	pub fn tier_2_card_pool(&self) -> &Vec<Card> {
		self.cards.tier_2()
	}
	pub fn tier_3_card_pool(&self) -> &Vec<Card> {
		self.cards.tier_3()
	}
	pub fn starting_player_index(&self) -> PlayerIndex {
		self.starting_player_index
	}
	pub fn number_of_players(&self) -> usize {
		self.number_of_players.count()
	}
	pub fn current_player(&self) -> &Player {
		&self.players[self.current_player_index]
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::gem::Gem;

	fn game_fixture() -> Game {
		let game_config = GameConfig {
			number_of_players: NumberOfPlayers::Three,
			seed: None,
		};
		Game::from_config(game_config)
	}

	#[test]
	fn test_from_config() {
		let game = game_fixture();

		assert_eq!(game.turn, 1);
		assert!(!game.cards.tier_1().is_empty());
		assert!(!game.cards.tier_2().is_empty());
		assert!(!game.cards.tier_3().is_empty());
		assert!(game.starting_player_index < game.players.len());
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(game.current_player_index, game.starting_player_index);
		assert_eq!(game.number_of_players, NumberOfPlayers::Three);
		assert_eq!(game.players.len(), NumberOfPlayers::Three.count());
		assert_ne!(game.token_pool.total(), 0);
		assert_ne!(game.aristocrats.len(), 0);
	}

	#[test]
	fn test_handle_action_take_two_gems() {
		let mut game = game_fixture();
		let player_index = game.current_player_index;

		assert_eq!(game.gem_pool().diamonds(), 5);
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(game.current_player().gems().diamonds(), 0);

		game.handle_action(Action::TakeTwoGems(Gem::Diamond)).unwrap();
		let player = &game.players[player_index];

		assert_ne!(game.current_player_index, player_index);
		assert_eq!(game.gem_pool().diamonds(), 3);
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(player.gems().diamonds(), 2);
	}

	#[test]
	fn test_handle_action_take_two_gems_failure() {
		let mut game = game_fixture();
		game.token_pool = (3, 5, 5, 5, 5, 0).into();
		let player_index = game.current_player_index;
		let turn = game.turn;

		assert_eq!(game.gem_pool().diamonds(), 3);
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(game.current_player().gems().diamonds(), 0);

		let result = game.handle_action(Action::TakeTwoGems(Gem::Diamond));
		assert_eq!(result, Err(HandleActionError::NotEnoughGems));
		let player = &game.players[player_index];

		assert_eq!(game.current_player_index, player_index);
		assert_eq!(game.turn, turn);
		assert_eq!(game.gem_pool().diamonds(), 3);
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(player.gems().diamonds(), 0);
	}

	#[test]
	fn test_handle_action_take_three_gems() {
		let mut game = game_fixture();
		let player_index = game.current_player_index;

		assert_eq!(game.gem_pool().diamonds(), 5);
		assert_eq!(game.gem_pool().sapphires(), 5);
		assert_eq!(game.gem_pool().rubies(), 5);
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(game.current_player().gems().diamonds(), 0);
		assert_eq!(game.current_player().gems().sapphires(), 0);
		assert_eq!(game.current_player().gems().rubies(), 0);

		game.handle_action(Action::TakeThreeGems(Gem::Diamond, Gem::Sapphire, Gem::Ruby)).unwrap();
		let player = &game.players[player_index];

		assert_ne!(game.current_player_index, player_index);
		assert_eq!(game.gem_pool().diamonds(), 4);
		assert_eq!(game.gem_pool().sapphires(), 4);
		assert_eq!(game.gem_pool().rubies(), 4);
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(player.gems().diamonds(), 1);
		assert_eq!(player.gems().sapphires(), 1);
		assert_eq!(player.gems().rubies(), 1);
	}

	#[test]
	fn test_handle_action_take_three_gems_failure() {
		let mut game = game_fixture();
		game.token_pool = (0, 1, 2, 3, 4, 2).into();
		let player_index = game.current_player_index;
		let turn = game.turn;

		assert_eq!(game.gem_pool().diamonds(), 0);
		assert_eq!(game.gem_pool().sapphires(), 1);
		assert_eq!(game.gem_pool().rubies(), 3);
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(game.current_player().gems().diamonds(), 0);
		assert_eq!(game.current_player().gems().sapphires(), 0);
		assert_eq!(game.current_player().gems().rubies(), 0);

		let result = game.handle_action(Action::TakeThreeGems(Gem::Diamond, Gem::Sapphire, Gem::Ruby));
		assert_eq!(result, Err(HandleActionError::NotEnoughGems));
		let player = &game.players[player_index];

		assert_eq!(game.current_player_index, player_index);
		assert_eq!(game.turn, turn);
		assert_eq!(game.gem_pool().diamonds(), 0);
		assert_eq!(game.gem_pool().sapphires(), 1);
		assert_eq!(game.gem_pool().rubies(), 3);
		assert_eq!(game.game_state, GameState::AwaitingAction);
		assert_eq!(player.gems().diamonds(), 0);
		assert_eq!(player.gems().sapphires(), 0);
		assert_eq!(player.gems().rubies(), 0);
	}
}
