use rand::Rng;

use crate::aristocrat::Aristocrat;
use crate::game_config::NumberOfPlayers;
use crate::gem_pool::GemPoolTuple;

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

#[derive(Debug, Clone)]
pub struct Aristocrats(Vec<Aristocrat>);

impl Aristocrats {
	pub fn deal(rng: &mut impl Rng, number_of_players: NumberOfPlayers) -> Self {
		use rand::prelude::SliceRandom;

		let aristocrats = ARISTOCRATS
			.choose_multiple(rng, number_of_players.number_of_aristocrats() as usize)
			.map(|&aristocrat_data| Aristocrat::from(aristocrat_data))
			.collect();

		Self(aristocrats)
	}
}

impl std::ops::Deref for Aristocrats {
	type Target = Vec<Aristocrat>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for Aristocrats {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_deal() {
		let mut rng = rand::thread_rng();

		let aristocrats = Aristocrats::deal(&mut rng, NumberOfPlayers::Two);
		assert_eq!(aristocrats.len(), NumberOfPlayers::Two.number_of_aristocrats() as usize);

		let aristocrats = Aristocrats::deal(&mut rng, NumberOfPlayers::Three);
		assert_eq!(
			aristocrats.len(),
			NumberOfPlayers::Three.number_of_aristocrats() as usize
		);

		let aristocrats = Aristocrats::deal(&mut rng, NumberOfPlayers::Four);
		assert_eq!(
			aristocrats.len(),
			NumberOfPlayers::Four.number_of_aristocrats() as usize
		);
	}
}
