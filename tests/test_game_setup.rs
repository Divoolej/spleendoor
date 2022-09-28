use spleendoor::{
	game::Game,
	game_config::{GameConfig, NumberOfPlayers},
};

fn test_common_setup(game: &Game) {
	assert_eq!(game.gold_pool(), 5);
	assert_eq!(game.tier_1_card_pool().len(), 40);
	assert_eq!(game.tier_2_card_pool().len(), 30);
	assert_eq!(game.tier_3_card_pool().len(), 20);
	assert_eq!(game.current_player_index(), game.starting_player_index());

	for i in 0..game.number_of_players() {
		let player = &game.players()[i];
		assert_eq!(player.token_count(), 0);
		assert_eq!(player.cards().len(), 0);
		assert_eq!(player.reserved_cards().len(), 0);
	}
}

#[test]
fn test_game_setup_for_two_players() {
	let game_config = GameConfig {
		number_of_players: NumberOfPlayers::Two,
		seed: None,
	};
	let game = Game::from_config(game_config);
	assert!(game.starting_player_index() < 2);
	assert_eq!(game.players().len(), 2);
	assert_eq!(game.number_of_players(), 2);
	assert_eq!(game.gem_pool().diamonds(), 4);
	assert_eq!(game.gem_pool().sapphires(), 4);
	assert_eq!(game.gem_pool().emeralds(), 4);
	assert_eq!(game.gem_pool().rubies(), 4);
	assert_eq!(game.gem_pool().onyxes(), 4);
	assert_eq!(game.aristocrat_pool().len(), 3);
	test_common_setup(&game);
}

#[test]
fn test_game_setup_for_three_players() {
	let game_config = GameConfig {
		number_of_players: NumberOfPlayers::Three,
		seed: None,
	};
	let game = Game::from_config(game_config);
	assert!(game.starting_player_index() < 3);
	assert_eq!(game.players().len(), 3);
	assert_eq!(game.number_of_players(), 3);
	assert_eq!(game.gem_pool().diamonds(), 5);
	assert_eq!(game.gem_pool().sapphires(), 5);
	assert_eq!(game.gem_pool().emeralds(), 5);
	assert_eq!(game.gem_pool().rubies(), 5);
	assert_eq!(game.gem_pool().onyxes(), 5);
	assert_eq!(game.aristocrat_pool().len(), 4);
	test_common_setup(&game);
}

#[test]
fn test_game_setup_for_four_players() {
	let game_config = GameConfig {
		number_of_players: NumberOfPlayers::Four,
		seed: None,
	};
	let game = Game::from_config(game_config);
	assert!(game.starting_player_index() < 4);
	assert_eq!(game.players().len(), 4);
	assert_eq!(game.number_of_players(), 4);
	assert_eq!(game.gem_pool().diamonds(), 7);
	assert_eq!(game.gem_pool().sapphires(), 7);
	assert_eq!(game.gem_pool().emeralds(), 7);
	assert_eq!(game.gem_pool().rubies(), 7);
	assert_eq!(game.gem_pool().onyxes(), 7);
	assert_eq!(game.aristocrat_pool().len(), 5);
	test_common_setup(&game);
}
