use crate::aristocrat::Aristocrat;
use crate::card::Card;
use crate::gem::Gem;
use crate::gem_pool::GemPool;
use crate::token_pool::TokenPool;

pub type PlayerIndex = usize;

#[derive(Debug, Clone)]
pub struct Player {
	tokens: TokenPool,
	cards: Vec<Card>,
	reserved_cards: Vec<Card>,
	aristocrats: Vec<Aristocrat>,
}

impl Player {
	pub fn new() -> Self {
		Self {
			tokens: TokenPool::empty(),
			cards: Vec::new(),
			reserved_cards: Vec::new(),
			aristocrats: Vec::new(),
		}
	}

	pub fn token_count(&self) -> u8 {
		self.tokens.total()
	}
	pub fn gems(&self) -> &GemPool {
		self.tokens.gems()
	}
	pub fn cards(&self) -> &Vec<Card> {
		&self.cards
	}
	pub fn cards_mut(&mut self) -> &mut Vec<Card> {
		&mut self.cards
	}
	pub fn reserved_cards(&self) -> &Vec<Card> {
		&self.reserved_cards
	}

	pub fn add_gem(&mut self, gem: Gem, count: u8) {
		self.tokens.gems_mut().add(gem, count);
	}

	pub fn remove_gem(&mut self, gem: Gem, count: u8) {
		self.tokens.gems_mut().remove(gem, count);
	}

	pub fn add_gold(&mut self) {
		*self.tokens.gold_mut() += 1;
	}

	pub fn remove_gold(&mut self, count: u8) {
		*self.tokens.gold_mut() = self.tokens.gold_mut().saturating_sub(count);
	}

	pub fn cards_gem_pool(&self) -> GemPool {
		self.cards.iter().fold(GemPool::new(0), |mut acc, next| {
			acc.add(next.gem(), 1);
			acc
		})
	}

	pub fn effective_gem_pool(&self) -> GemPool {
		GemPool::union(&self.cards_gem_pool(), self.tokens.gems())
	}

	pub fn can_buy(&self, card: &Card) -> bool {
		let difference = GemPool::difference(card.cost(), &self.effective_gem_pool());
		difference.total() <= self.tokens.gold()
	}
}

impl Default for Player {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let player = Player::new();
		assert_eq!(player.tokens.total(), 0);
		assert_eq!(player.cards.len(), 0);
		assert_eq!(player.reserved_cards.len(), 0);
	}

	#[test]
	fn test_add_gem() {
		let mut player = Player::new();
		assert_eq!(player.token_count(), 0);
		player.add_gem(Gem::Diamond, 1);
		assert_eq!(player.token_count(), 1);
		player.add_gem(Gem::Ruby, 2);
		assert_eq!(player.token_count(), 3);
	}

	#[test]
	fn test_remove_gem() {
		let mut player = Player::new();
		player.add_gem(Gem::Diamond, 1);
		player.add_gem(Gem::Ruby, 2);
		assert_eq!(player.token_count(), 3);
		player.remove_gem(Gem::Diamond, 1);
		assert_eq!(player.token_count(), 2);
		player.remove_gem(Gem::Ruby, 2);
		assert_eq!(player.token_count(), 0);
	}
}
