use crate::aristocrat::Aristocrat;
use crate::gem_pool::GemPoolTuple;
use crate::game_config::NumberOfPlayers;

type AristocratData = GemPoolTuple;

static ARISTOCRATS: &[AristocratData] = &[
	(3, 3, 3, 0, 0),
	(0, 3, 3, 3, 0),
	(0, 0, 3, 3, 3),
	(3, 0, 0, 3, 3),
	(3, 3, 0, 0, 3),
	(4, 4, 0, 0, 0),
	(0, 4, 4, 0, 0),
	(0, 0, 4, 4, 0),
	(0, 0, 0, 4, 4),
	(4, 0, 0, 0, 4),
];

pub struct Aristocrats(Vec<Aristocrat>);

impl Aristocrats {
	pub fn deal(number_of_players: NumberOfPlayers) -> Self {
		use rand::prelude::SliceRandom;

		let aristocrats = ARISTOCRATS
				.choose_multiple(&mut rand::thread_rng(), number_of_players.number_of_aristocrats() as usize)
				.map(|&aristocrat_data| Aristocrat::from(aristocrat_data))
				.collect();

		Self(aristocrats)
	}
}