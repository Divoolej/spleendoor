use crate::card::Tier;
use crate::gem::Gem;

#[derive(Debug, Clone, Copy)]
pub enum Action {
	TakeTwoGems(Gem),
	TakeThreeGems(Gem, Gem, Gem),
	BuyCard(Tier, u8),
	BuyReservedCard(u8),
	ReserveCard(Tier, u8),
	Pass,
}
