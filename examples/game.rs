use spleendoor::prelude::{Game, GameConfig, NumberOfPlayers};

fn main() {
	let cfg = GameConfig {
		number_of_players: NumberOfPlayers::Three,
		seed: Some("EXAMPL".to_owned()),
	};
	let mut game = Game::from_config(cfg);
	println!("{:#?}", game.board_state());
}
