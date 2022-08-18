use crate::gem::Gem;
use crate::gem_pool::GemPool;

#[derive(Debug, Clone, Copy)]
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
}
