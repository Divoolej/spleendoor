use spleendoor::{
	game_config::{GameConfig, NumberOfPlayers},
	game::Game,
};

#[test]
fn test_game_setup_for_two_players() {
	let game_config = GameConfig { number_of_players: NumberOfPlayers::Two };
	let game = Game::from_game_config(&game_config);
	assert_eq!(game.number_of_players(), 2);
	assert_eq!(game.gold_left(), 5);
	assert_eq!(game.gems_left().diamonds(), 4);
	assert_eq!(game.gems_left().sapphires(), 4);
	assert_eq!(game.gems_left().emeralds(), 4);
	assert_eq!(game.gems_left().rubies(), 4);
	assert_eq!(game.gems_left().onyxes(), 4);
	assert_eq!(game.aristocrats_left().len(), 3);
	assert_eq!(game.tier_1_cards_left().len(), 40);
	assert_eq!(game.tier_2_cards_left().len(), 30);
	assert_eq!(game.tier_3_cards_left().len(), 20);
}

#[test]
fn test_game_setup_for_three_players() {
	let game_config = GameConfig { number_of_players: NumberOfPlayers::Three };
	let game = Game::from_game_config(&game_config);
	assert_eq!(game.number_of_players(), 3);
	assert_eq!(game.gold_left(), 5);
	assert_eq!(game.gems_left().diamonds(), 5);
	assert_eq!(game.gems_left().sapphires(), 5);
	assert_eq!(game.gems_left().emeralds(), 5);
	assert_eq!(game.gems_left().rubies(), 5);
	assert_eq!(game.gems_left().onyxes(), 5);
	assert_eq!(game.aristocrats_left().len(), 4);
	assert_eq!(game.tier_1_cards_left().len(), 40);
	assert_eq!(game.tier_2_cards_left().len(), 30);
	assert_eq!(game.tier_3_cards_left().len(), 20);
}

#[test]
fn test_game_setup_for_four_players() {
	let game_config = GameConfig { number_of_players: NumberOfPlayers::Four };
	let game = Game::from_game_config(&game_config);
	assert_eq!(game.number_of_players(), 4);
	assert_eq!(game.gold_left(), 5);
	assert_eq!(game.gems_left().diamonds(), 7);
	assert_eq!(game.gems_left().sapphires(), 7);
	assert_eq!(game.gems_left().emeralds(), 7);
	assert_eq!(game.gems_left().rubies(), 7);
	assert_eq!(game.gems_left().onyxes(), 7);
	assert_eq!(game.aristocrats_left().len(), 5);
	assert_eq!(game.tier_1_cards_left().len(), 40);
	assert_eq!(game.tier_2_cards_left().len(), 30);
	assert_eq!(game.tier_3_cards_left().len(), 20);
}