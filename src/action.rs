use crate::gem::Gem;
use crate::card::Tier;

pub enum Action {
	TakeTwoGems(Gem),
	TakeThreeGems(Gem, Gem, Gem),
	BuyCard(Tier, u8),
	BuyReservedCard(u8),
	ReserveCard(Tier, u8),
	Pass,
}
