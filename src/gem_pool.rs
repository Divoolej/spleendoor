use crate::gem::Gem;

pub type GemPoolTuple = (u8, u8, u8, u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GemPool {
	diamonds: u8,
	sapphires: u8,
	emeralds: u8,
	rubies: u8,
	onyxes: u8,
}

impl GemPool {
	pub fn union(first: &GemPool, second: &GemPool) -> GemPool {
		(
			first.diamonds + second.diamonds,
			first.sapphires + second.sapphires,
			first.emeralds + second.emeralds,
			first.rubies + second.rubies,
			first.onyxes + second.onyxes,
		)
			.into()
	}

	pub fn difference(first: &GemPool, second: &GemPool) -> GemPool {
		(
			first.diamonds.saturating_sub(second.diamonds),
			first.sapphires.saturating_sub(second.sapphires),
			first.emeralds.saturating_sub(second.emeralds),
			first.rubies.saturating_sub(second.rubies),
			first.onyxes.saturating_sub(second.onyxes),
		)
			.into()
	}

	pub fn new(initial_amount: u8) -> Self {
		(
			initial_amount,
			initial_amount,
			initial_amount,
			initial_amount,
			initial_amount,
		)
			.into()
	}

	pub fn total(&self) -> u8 {
		self.diamonds + self.sapphires + self.emeralds + self.rubies + self.onyxes
	}

	pub fn count(&self, gem: Gem) -> u8 {
		match gem {
			Gem::Diamond => self.diamonds,
			Gem::Sapphire => self.sapphires,
			Gem::Emerald => self.emeralds,
			Gem::Ruby => self.rubies,
			Gem::Onyx => self.onyxes,
		}
	}

	pub fn add(&mut self, gem: Gem, count: u8) {
		match gem {
			Gem::Diamond => self.diamonds += count,
			Gem::Sapphire => self.sapphires += count,
			Gem::Emerald => self.emeralds += count,
			Gem::Ruby => self.rubies += count,
			Gem::Onyx => self.onyxes += count,
		}
	}

	pub fn remove(&mut self, gem: Gem, count: u8) {
		match gem {
			Gem::Diamond => self.diamonds -= count,
			Gem::Sapphire => self.sapphires -= count,
			Gem::Emerald => self.emeralds -= count,
			Gem::Ruby => self.rubies -= count,
			Gem::Onyx => self.onyxes -= count,
		}
	}

	pub fn diamonds(&self) -> u8 {
		self.diamonds
	}
	pub fn sapphires(&self) -> u8 {
		self.sapphires
	}
	pub fn emeralds(&self) -> u8 {
		self.emeralds
	}
	pub fn rubies(&self) -> u8 {
		self.rubies
	}
	pub fn onyxes(&self) -> u8 {
		self.onyxes
	}
}

impl From<GemPoolTuple> for GemPool {
	fn from((diamonds, sapphires, emeralds, rubies, onyxes): GemPoolTuple) -> Self {
		Self {
			diamonds,
			sapphires,
			emeralds,
			rubies,
			onyxes,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let gem_pool = GemPool::new(2);
		assert_eq!(gem_pool.diamonds, 2);
		assert_eq!(gem_pool.sapphires, 2);
		assert_eq!(gem_pool.emeralds, 2);
		assert_eq!(gem_pool.rubies, 2);
		assert_eq!(gem_pool.onyxes, 2);
	}

	#[test]
	fn test_total() {
		let gem_pool = GemPool {
			diamonds: 1,
			sapphires: 2,
			emeralds: 3,
			rubies: 4,
			onyxes: 5,
		};
		assert_eq!(gem_pool.total(), 15);
	}

	#[test]
	fn test_count() {
		let gem_pool = GemPool {
			diamonds: 1,
			sapphires: 2,
			emeralds: 3,
			rubies: 4,
			onyxes: 5,
		};
		assert_eq!(gem_pool.count(Gem::Diamond), 1);
		assert_eq!(gem_pool.count(Gem::Sapphire), 2);
		assert_eq!(gem_pool.count(Gem::Emerald), 3);
		assert_eq!(gem_pool.count(Gem::Ruby), 4);
		assert_eq!(gem_pool.count(Gem::Onyx), 5);
	}

	#[test]
	fn test_add() {
		let mut gem_pool = GemPool::new(0);
		gem_pool.add(Gem::Diamond, 2);
		gem_pool.add(Gem::Ruby, 1);
		assert_eq!(gem_pool.diamonds, 2);
		assert_eq!(gem_pool.sapphires, 0);
		assert_eq!(gem_pool.emeralds, 0);
		assert_eq!(gem_pool.rubies, 1);
		assert_eq!(gem_pool.onyxes, 0);
	}

	#[test]
	fn test_remove() {
		let mut gem_pool = GemPool::new(2);
		gem_pool.remove(Gem::Diamond, 2);
		gem_pool.remove(Gem::Ruby, 1);
		assert_eq!(gem_pool.diamonds, 0);
		assert_eq!(gem_pool.sapphires, 2);
		assert_eq!(gem_pool.emeralds, 2);
		assert_eq!(gem_pool.rubies, 1);
		assert_eq!(gem_pool.onyxes, 2);
	}

	#[test]
	fn test_from_gem_pool_tuple() {
		let gem_pool_tuple = (1, 2, 3, 4, 5);
		let gem_pool: GemPool = gem_pool_tuple.into();
		assert_eq!(gem_pool.diamonds, 1);
		assert_eq!(gem_pool.sapphires, 2);
		assert_eq!(gem_pool.emeralds, 3);
		assert_eq!(gem_pool.rubies, 4);
		assert_eq!(gem_pool.onyxes, 5);
	}

	#[test]
	fn test_union() {
		let a: GemPool = (1, 2, 3, 4, 5).into();
		let b: GemPool = (5, 4, 3, 2, 1).into();

		assert_eq!(GemPool::union(&a, &b), (6, 6, 6, 6, 6).into());
	}

	#[test]
	fn test_difference() {
		let a: GemPool = (1, 2, 3, 4, 5).into();
		let b: GemPool = (0, 1, 3, 5, 7).into();

		assert_eq!(GemPool::difference(&a, &b), (1, 1, 0, 0, 0).into());
	}
}
