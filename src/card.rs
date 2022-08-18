use crate::gem::Gem;
use crate::gem_pool::GemPool;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tier { One, Two, Three }

pub type Points = u8;

#[derive(Debug, Clone)]
pub struct Card {
	tier: Tier,
	gem: Gem,
	cost: GemPool,
	points: Points,
}

impl Card {
	pub fn new(tier: Tier, gem: Gem, points: Points, cost: GemPool) -> Self {
		Self { tier, gem, points, cost }
	}

	pub fn gem(&self) -> Gem { self.gem }
	pub fn tier(&self) -> Tier { self.tier }
	pub fn cost(&self) -> &GemPool { &self.cost }
	pub fn points(&self) -> Points { self.points }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let card = Card::new(Tier::Two, Gem::Ruby, 2, (1, 2, 3, 4, 5).into());
		assert_eq!(card.tier, Tier::Two);
		assert_eq!(card.gem, Gem::Ruby);
		assert_eq!(card.points, 2);
		assert_eq!(card.cost.diamonds(), 1);
		assert_eq!(card.cost.sapphires(), 2);
		assert_eq!(card.cost.emeralds(), 3);
		assert_eq!(card.cost.rubies(), 4);
		assert_eq!(card.cost.onyxes(), 5);
	}
}