use spleendoor::{
	game_config::{GameConfig, NumberOfPlayers},
	game::Game,
};

fn test_common_setup(game: &Game) {
	assert_eq!(game.gold_pool(), 5);
	assert_eq!(game.tier_1_card_pool().len(), 40);
	assert_eq!(game.tier_2_card_pool().len(), 30);
	assert_eq!(game.tier_3_card_pool().len(), 20);
	assert_eq!(game.current_player(), game.starting_player());
}

#[test]
fn test_game_setup_for_two_players() {
	let game_config = GameConfig { number_of_players: NumberOfPlayers::Two };
	let game = Game::from_game_config(&game_config);
	test_common_setup(&game);
	assert!(game.starting_player() < 2);
	assert_eq!(game.players().len(), 2);
	assert_eq!(game.number_of_players(), 2);
	assert_eq!(game.gem_pool().diamonds(), 4);
	assert_eq!(game.gem_pool().sapphires(), 4);
	assert_eq!(game.gem_pool().emeralds(), 4);
	assert_eq!(game.gem_pool().rubies(), 4);
	assert_eq!(game.gem_pool().onyxes(), 4);
	assert_eq!(game.aristocrat_pool().len(), 3);
}

#[test]
fn test_game_setup_for_three_players() {
	let game_config = GameConfig { number_of_players: NumberOfPlayers::Three };
	let game = Game::from_game_config(&game_config);
	test_common_setup(&game);
	assert!(game.starting_player() < 3);
	assert_eq!(game.players().len(), 3);
	assert_eq!(game.number_of_players(), 3);
	assert_eq!(game.gem_pool().diamonds(), 5);
	assert_eq!(game.gem_pool().sapphires(), 5);
	assert_eq!(game.gem_pool().emeralds(), 5);
	assert_eq!(game.gem_pool().rubies(), 5);
	assert_eq!(game.gem_pool().onyxes(), 5);
	assert_eq!(game.aristocrat_pool().len(), 4);
}

#[test]
fn test_game_setup_for_four_players() {
	let game_config = GameConfig { number_of_players: NumberOfPlayers::Four };
	let game = Game::from_game_config(&game_config);
	test_common_setup(&game);
	assert!(game.starting_player() < 4);
	assert_eq!(game.players().len(), 4);
	assert_eq!(game.number_of_players(), 4);
	assert_eq!(game.gem_pool().diamonds(), 7);
	assert_eq!(game.gem_pool().sapphires(), 7);
	assert_eq!(game.gem_pool().emeralds(), 7);
	assert_eq!(game.gem_pool().rubies(), 7);
	assert_eq!(game.gem_pool().onyxes(), 7);
	assert_eq!(game.aristocrat_pool().len(), 5);
}