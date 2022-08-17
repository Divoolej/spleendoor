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

pub struct Cards {
	tier_1: Vec<Card>,
	tier_2: Vec<Card>,
	tier_3: Vec<Card>,
}

impl Cards {
	fn deal_tier(tier: Tier, cards: &[CardData]) -> Vec<Card> {
		use rand::prelude::SliceRandom;

		let mut deck: Vec<Card> = cards.into_iter().map(|&(gem, points, cost)| {
			Card::new(tier, gem, points, cost.into())
		}).collect();

		deck.shuffle(&mut rand::thread_rng());

		deck
	}

	pub fn deal() -> Self {
		Self {
			tier_1: Self::deal_tier(Tier::One, TIER_1_CARDS),
			tier_2: Self::deal_tier(Tier::Two, TIER_2_CARDS),
			tier_3: Self::deal_tier(Tier::Three, TIER_3_CARDS),
		}
	}
}
