use env_logger;
use spleendoor::prelude::{Action::*, Game, GameConfig, Gem::*, NumberOfPlayers, Tier};

fn main() {
	env_logger::init();

	let cfg = GameConfig {
		number_of_players: NumberOfPlayers::Three,
		seed: Some("EXAMPL".to_owned()),
	};
	let mut game = Game::from_config(cfg);
	/* Player 1 */
	game.handle_action(TakeThreeGems(Sapphire, Ruby, Onyx)).unwrap();
	/* Player 2 */
	game.handle_action(TakeTwoGems(Ruby)).unwrap();
	/* Player 3 */
	game.handle_action(TakeThreeGems(Emerald, Ruby, Onyx)).unwrap();
	/* Player 1 */
	game.handle_action(TakeThreeGems(Sapphire, Diamond, Onyx)).unwrap();
	/* Player 2 */
	game.handle_action(TakeTwoGems(Emerald)).unwrap();
	/* Player 3 */
	game.handle_action(TakeThreeGems(Emerald, Ruby, Onyx)).unwrap();
	#[cfg(debug_assertions)] game.board_state().pretty_print();
	game.handle_action(BuyCard(Tier::One, 1)).unwrap();
	#[cfg(debug_assertions)] game.board_state().pretty_print();
}
