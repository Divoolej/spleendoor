use rand::Rng;

use crate::gem::Gem;
use crate::gem_pool::GemPoolTuple;
use crate::card::{Tier, Card, Points};

type CardData = (Gem, Points, GemPoolTuple);

static TIER_1_CARDS: &[CardData] = &[
	(Gem::Diamond, 0, (0, 0, 0, 2, 1)),
	(Gem::Diamond, 0, (0, 3, 0, 0, 0)),
	(Gem::Diamond, 0, (0, 2, 0, 0, 2)),
	(Gem::Diamond, 0, (0, 1, 1, 1, 1)),
	(Gem::Diamond, 0, (3, 1, 0, 0, 1)),
	(Gem::Diamond, 0, (0, 1, 2, 1, 1)),
	(Gem::Diamond, 0, (0, 2, 2, 0, 1)),
	(Gem::Diamond, 1, (0, 0, 4, 0, 0)),
	(Gem::Sapphire, 0, (1, 0, 0, 0, 2)),
	(Gem::Sapphire, 0, (0, 0, 0, 0, 3)),
	(Gem::Sapphire, 0, (0, 0, 2, 0, 2)),
	(Gem::Sapphire, 0, (1, 0, 1, 1, 1)),
	(Gem::Sapphire, 0, (0, 1, 3, 1, 0)),
	(Gem::Sapphire, 0, (1, 0, 1, 2, 1)),
	(Gem::Sapphire, 0, (1, 0, 2, 2, 0)),
	(Gem::Sapphire, 1, (0, 0, 0, 4, 0)),
	(Gem::Emerald, 0, (2, 1, 0, 0, 0)),
	(Gem::Emerald, 0, (0, 0, 0, 3, 0)),
	(Gem::Emerald, 0, (0, 2, 0, 2, 0)),
	(Gem::Emerald, 0, (1, 1, 0, 1, 1)),
	(Gem::Emerald, 0, (1, 3, 1, 0, 0)),
	(Gem::Emerald, 0, (1, 1, 0, 1, 2)),
	(Gem::Emerald, 0, (1, 0, 0, 2, 2)),
	(Gem::Emerald, 1, (0, 0, 0, 0, 4)),
	(Gem::Ruby,    0, (0, 2, 1, 0, 0)),
	(Gem::Ruby,    0, (3, 0, 0, 0, 0)),
	(Gem::Ruby,    0, (2, 0, 0, 2, 0)),
	(Gem::Ruby,    0, (1, 1, 1, 0, 1)),
	(Gem::Ruby,    0, (1, 0, 0, 1, 3)),
	(Gem::Ruby,    0, (2, 1, 1, 0, 1)),
	(Gem::Ruby,    0, (2, 0, 1, 0, 2)),
	(Gem::Ruby,    1, (4, 0, 0, 0, 0)),
	(Gem::Onyx,    0, (0, 0, 2, 1, 0)),
	(Gem::Onyx,    0, (0, 0, 3, 0, 0)),
	(Gem::Onyx,    0, (2, 0, 2, 0, 0)),
	(Gem::Onyx,    0, (1, 1, 1, 1, 0)),
	(Gem::Onyx,    0, (0, 0, 1, 3, 1)),
	(Gem::Onyx,    0, (1, 2, 1, 1, 0)),
	(Gem::Onyx,    0, (2, 2, 0, 1, 0)),
	(Gem::Onyx,    1, (0, 4, 0, 0, 0)),
];

static TIER_2_CARDS: &[CardData] = &[
	(Gem::Diamond, 1, (0, 0, 3, 2, 2)),
	(Gem::Diamond, 1, (2, 3, 0, 3, 0)),
	(Gem::Diamond, 2, (0, 0, 0, 5, 0)),
	(Gem::Diamond, 2, (0, 0, 1, 4, 2)),
	(Gem::Diamond, 2, (0, 0, 0, 5, 3)),
	(Gem::Diamond, 3, (6, 0, 0, 0, 0)),
	(Gem::Sapphire, 1, (0, 2, 2, 3, 0)),
	(Gem::Sapphire, 1, (0, 2, 3, 0, 3)),
	(Gem::Sapphire, 2, (0, 5, 0, 0, 0)),
	(Gem::Sapphire, 2, (2, 0, 0, 1, 4)),
	(Gem::Sapphire, 2, (5, 3, 0, 0, 0)),
	(Gem::Sapphire, 3, (0, 6, 0, 0, 0)),
	(Gem::Emerald, 1, (2, 3, 0, 0, 2)),
	(Gem::Emerald, 1, (3, 0, 2, 3, 0)),
	(Gem::Emerald, 2, (0, 0, 5, 0, 0)),
	(Gem::Emerald, 2, (4, 2, 0, 0, 1)),
	(Gem::Emerald, 2, (0, 5, 3, 0, 0)),
	(Gem::Emerald, 3, (0, 0, 6, 0, 0)),
	(Gem::Ruby,    1, (2, 0, 0, 2, 3)),
	(Gem::Ruby,    1, (0, 3, 0, 2, 3)),
	(Gem::Ruby,    2, (0, 0, 0, 0, 5)),
	(Gem::Ruby,    2, (1, 4, 2, 0, 0)),
	(Gem::Ruby,    2, (3, 0, 0, 0, 5)),
	(Gem::Ruby,    3, (0, 0, 0, 6, 0)),
	(Gem::Onyx,    1, (3, 2, 2, 0, 0)),
	(Gem::Onyx,    1, (3, 0, 3, 0, 2)),
	(Gem::Onyx,    2, (5, 0, 0, 0, 0)),
	(Gem::Onyx,    2, (0, 1, 4, 2, 0)),
	(Gem::Onyx,    2, (0, 0, 5, 3, 0)),
	(Gem::Onyx,    3, (0, 0, 0, 0, 6)),
];

static TIER_3_CARDS: &[CardData] = &[
	(Gem::Diamond, 3, (0, 3, 3, 5, 3)),
	(Gem::Diamond, 4, (0, 0, 0, 0, 7)),
	(Gem::Diamond, 4, (3, 0, 0, 3, 6)),
	(Gem::Diamond, 5, (3, 0, 0, 0, 7)),
	(Gem::Sapphire, 3, (3, 0, 3, 3, 5)),
	(Gem::Sapphire, 4, (7, 0, 0, 0, 0)),
	(Gem::Sapphire, 4, (6, 3, 0, 0, 3)),
	(Gem::Sapphire, 5, (7, 3, 0, 0, 0)),
	(Gem::Emerald, 3, (5, 3, 0, 3, 3)),
	(Gem::Emerald, 4, (0, 7, 0, 0, 0)),
	(Gem::Emerald, 4, (3, 6, 3, 0, 0)),
	(Gem::Emerald, 5, (0, 7, 3, 0, 0)),
	(Gem::Ruby,    3, (3, 5, 3, 0, 3)),
	(Gem::Ruby,    4, (0, 0, 7, 0, 0)),
	(Gem::Ruby,    4, (0, 3, 6, 3, 0)),
	(Gem::Ruby,    5, (0, 0, 7, 3, 0)),
	(Gem::Onyx,    3, (3, 3, 5, 3, 0)),
	(Gem::Onyx,    4, (0, 0, 0, 7, 0)),
	(Gem::Onyx,    4, (0, 0, 3, 6, 3)),
	(Gem::Onyx,    5, (0, 0, 0, 7, 3)),
];

#[derive(Debug)]
pub struct Cards {
	tier_1: Vec<Card>,
	tier_2: Vec<Card>,
	tier_3: Vec<Card>,
}

impl Cards {
	fn deal_tier(rng: &mut impl Rng, tier: Tier, cards: &[CardData]) -> Vec<Card> {
		use rand::prelude::SliceRandom;

		let mut deck: Vec<Card> = cards.iter().map(|&(gem, points, cost)| {
			Card::new(tier, gem, points, cost.into())
		}).collect();

		deck.shuffle(rng);

		deck
	}

	pub fn deal(rng: &mut impl Rng) -> Self {
		Self {
			tier_1: Self::deal_tier(rng, Tier::One, TIER_1_CARDS),
			tier_2: Self::deal_tier(rng, Tier::Two, TIER_2_CARDS),
			tier_3: Self::deal_tier(rng, Tier::Three, TIER_3_CARDS),
		}
	}

	pub fn tier(&self, tier: Tier) -> &Vec<Card> {
		match tier {
			Tier::One => &self.tier_1,
			Tier::Two => &self.tier_2,
			Tier::Three => &self.tier_3,
		}
	}

	pub fn tier_mut(&mut self, tier: Tier) -> &mut Vec<Card> {
		match tier {
			Tier::One => &mut self.tier_1,
			Tier::Two => &mut self.tier_2,
			Tier::Three => &mut self.tier_3,
		}
	}

	pub fn visible(&self) -> Cards {
		Cards {
			tier_1: self.tier_1.iter().take(4).cloned().collect(),
			tier_2: self.tier_2.iter().take(4).cloned().collect(),
			tier_3: self.tier_3.iter().take(4).cloned().collect(),
		}
	}

	pub fn tier_1(&self) -> &Vec<Card> { &self.tier_1 }
	pub fn tier_2(&self) -> &Vec<Card> { &self.tier_2 }
	pub fn tier_3(&self) -> &Vec<Card> { &self.tier_3 }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_deal() {
		let cards = Cards::deal(&mut rand::thread_rng());
		assert_eq!(cards.tier_1.len(), 40);
		assert!(cards.tier_1.iter().all(|card| card.tier() == Tier::One));
		assert_eq!(cards.tier_2.len(), 30);
		assert!(cards.tier_2.iter().all(|card| card.tier() == Tier::Two));
		assert_eq!(cards.tier_3.len(), 20);
		assert!(cards.tier_3.iter().all(|card| card.tier() == Tier::Three));
	}

	#[test]
	fn test_visible_when_full() {
		let cards = Cards::deal(&mut rand::thread_rng());
		let visible_cards = cards.visible();
		assert_eq!(visible_cards.tier_1.len(), 4);
		assert_eq!(visible_cards.tier_2.len(), 4);
		assert_eq!(visible_cards.tier_3.len(), 4);
	}

	#[test]
	fn test_visible_when_empty() {
		let cards = Cards {
			tier_1: vec![],
			tier_2: vec![],
			tier_3: vec![],
		};
		let visible_cards = cards.visible();
		assert_eq!(visible_cards.tier_1.len(), 0);
		assert_eq!(visible_cards.tier_2.len(), 0);
		assert_eq!(visible_cards.tier_3.len(), 0);
	}
}
