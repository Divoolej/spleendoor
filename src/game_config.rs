#[derive(Clone, Copy)]
pub enum NumberOfPlayers { Two, Three, Four }

pub struct GameConfig {
	pub number_of_players: NumberOfPlayers,
}
