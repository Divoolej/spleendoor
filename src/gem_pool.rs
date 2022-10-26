use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::gem::Gem;

pub type GemPoolTuple = (u8, u8, u8, u8, u8);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GemPool {
	pub diamonds: u8,
	pub sapphires: u8,
	pub emeralds: u8,
	pub rubies: u8,
	pub onyxes: u8,
}

impl GemPool {
	pub fn empty() -> Self {
		(0, 0, 0, 0, 0).into()
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

impl From<Gem> for GemPool {
	fn from(gem: Gem) -> Self {
		1 * gem
	}
}

impl Add for GemPool {
	type Output = Self;

	fn add(self, rhs: GemPool) -> GemPool {
		GemPool {
			diamonds: self.diamonds + rhs.diamonds,
			sapphires: self.sapphires + rhs.sapphires,
			emeralds: self.emeralds + rhs.emeralds,
			rubies: self.rubies + rhs.rubies,
			onyxes: self.onyxes + rhs.onyxes,
		}
	}
}

impl Add<Gem> for GemPool {
	type Output = Self;

	fn add(self, rhs: Gem) -> GemPool {
		self + GemPool::from(rhs)
	}
}

impl AddAssign for GemPool {
	fn add_assign(&mut self, rhs: GemPool) {
		self.diamonds += rhs.diamonds;
		self.sapphires += rhs.sapphires;
		self.emeralds += rhs.emeralds;
		self.rubies += rhs.rubies;
		self.onyxes += rhs.onyxes;
	}
}

impl AddAssign<Gem> for GemPool {
	fn add_assign(&mut self, rhs: Gem) {
		*self = self.add(rhs);
	}
}

impl Sub for GemPool {
	type Output = Self;

	fn sub(self, rhs: GemPool) -> GemPool {
		GemPool {
			diamonds: self.diamonds.saturating_sub(rhs.diamonds),
			sapphires: self.sapphires.saturating_sub(rhs.sapphires),
			emeralds: self.emeralds.saturating_sub(rhs.emeralds),
			rubies: self.rubies.saturating_sub(rhs.rubies),
			onyxes: self.onyxes.saturating_sub(rhs.onyxes),
		}
	}
}

impl Sub<Gem> for GemPool {
	type Output = Self;

	fn sub(self, rhs: Gem) -> GemPool {
		self - GemPool::from(rhs)
	}
}

impl SubAssign for GemPool {
	fn sub_assign(&mut self, rhs: GemPool) {
		self.diamonds = self.diamonds.saturating_sub(rhs.diamonds);
		self.sapphires = self.sapphires.saturating_sub(rhs.sapphires);
		self.emeralds = self.emeralds.saturating_sub(rhs.emeralds);
		self.rubies = self.rubies.saturating_sub(rhs.rubies);
		self.onyxes = self.onyxes.saturating_sub(rhs.onyxes);
	}
}

impl SubAssign<Gem> for GemPool {
	fn sub_assign(&mut self, rhs: Gem) {
		*self = self.sub(rhs);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_gem_pool_tuple() {
		let gem_pool = GemPool::from((1, 2, 3, 4, 5));
		assert_eq!(gem_pool.diamonds, 1);
		assert_eq!(gem_pool.sapphires, 2);
		assert_eq!(gem_pool.emeralds, 3);
		assert_eq!(gem_pool.rubies, 4);
		assert_eq!(gem_pool.onyxes, 5);
	}

	#[test]
	fn test_from_gem() {
		let gem_pool = GemPool::from(Gem::Sapphire);
		assert_eq!(gem_pool.diamonds, 0);
		assert_eq!(gem_pool.sapphires, 1);
		assert_eq!(gem_pool.emeralds, 0);
		assert_eq!(gem_pool.rubies, 0);
		assert_eq!(gem_pool.onyxes, 0);
	}

	#[test]
	fn test_default() {
		assert_eq!(GemPool::default(), (0, 0, 0, 0, 0).into());
	}

	#[test]
	fn test_empty() {
		assert_eq!(GemPool::empty(), (0, 0, 0, 0, 0).into());
	}

	#[test]
	fn test_new() {
		assert_eq!(GemPool::new(2), (2, 2, 2, 2, 2).into());
	}

	#[test]
	fn test_total() {
		let gem_pool = GemPool::from((1, 2, 3, 4, 5));
		assert_eq!(gem_pool.total(), 15);
	}

	#[test]
	fn test_count() {
		let gem_pool = GemPool::from((1, 2, 3, 4, 5));
		assert_eq!(gem_pool.count(Gem::Diamond), 1);
		assert_eq!(gem_pool.count(Gem::Sapphire), 2);
		assert_eq!(gem_pool.count(Gem::Emerald), 3);
		assert_eq!(gem_pool.count(Gem::Ruby), 4);
		assert_eq!(gem_pool.count(Gem::Onyx), 5);
	}

	#[test]
	fn test_add_gem_pool() {
		assert_eq!(
			GemPool::from((1, 2, 3, 4, 5)) + GemPool::from((5, 4, 3, 2, 1)),
			GemPool::from((6, 6, 6, 6, 6))
		);
	}

	#[test]
	fn test_sub_gem_pool() {
		assert_eq!(
			GemPool::from((1, 2, 3, 4, 5)) - GemPool::from((5, 1, 3, 2, 0)),
			GemPool::from((0, 1, 0, 2, 5))
		);
	}

	#[test]
	fn test_add_gem() {
		let gem_pool = GemPool::empty();
		assert_eq!(gem_pool + Gem::Diamond, (1, 0, 0, 0, 0).into());
		assert_eq!(gem_pool + Gem::Sapphire, (0, 1, 0, 0, 0).into());
		assert_eq!(gem_pool + Gem::Emerald, (0, 0, 1, 0, 0).into());
		assert_eq!(gem_pool + Gem::Ruby, (0, 0, 0, 1, 0).into());
		assert_eq!(gem_pool + Gem::Onyx, (0, 0, 0, 0, 1).into());
	}

	#[test]
	fn test_sub_gem() {
		let gem_pool = GemPool::from((1, 2, 3, 4, 5));
		assert_eq!(gem_pool - Gem::Diamond, (0, 2, 3, 4, 5).into());
		assert_eq!(gem_pool - Gem::Sapphire, (1, 1, 3, 4, 5).into());
		assert_eq!(gem_pool - Gem::Emerald, (1, 2, 2, 4, 5).into());
		assert_eq!(gem_pool - Gem::Ruby, (1, 2, 3, 3, 5).into());
		assert_eq!(gem_pool - Gem::Onyx, (1, 2, 3, 4, 4).into());
	}

	#[test]
	fn test_add_assign_gem_pool() {
		let mut gem_pool = GemPool::from((1, 2, 3, 4, 5));
		gem_pool += GemPool::from((1, 0, 3, 0, 5));
		assert_eq!(gem_pool, (2, 2, 6, 4, 10).into());
	}

	#[test]
	fn test_sub_assign_gem_pool() {
		let mut gem_pool = GemPool::from((1, 2, 3, 4, 5));
		gem_pool -= GemPool::from((5, 0, 3, 0, 1));
		assert_eq!(gem_pool, (0, 2, 0, 4, 4).into());
	}

	#[test]
	fn test_add_assign_gem() {
		let mut gem_pool = GemPool::from((1, 2, 3, 4, 5));
		gem_pool += Gem::Diamond;
		assert_eq!(gem_pool, (2, 2, 3, 4, 5).into());
		gem_pool += Gem::Sapphire;
		assert_eq!(gem_pool, (2, 3, 3, 4, 5).into());
		gem_pool += Gem::Emerald;
		assert_eq!(gem_pool, (2, 3, 4, 4, 5).into());
		gem_pool += Gem::Ruby;
		assert_eq!(gem_pool, (2, 3, 4, 5, 5).into());
		gem_pool += Gem::Onyx;
		assert_eq!(gem_pool, (2, 3, 4, 5, 6).into());
	}

	#[test]
	fn test_sub_assign_gem() {
		let mut gem_pool = GemPool::from((1, 2, 3, 4, 5));
		gem_pool -= Gem::Diamond;
		assert_eq!(gem_pool, (0, 2, 3, 4, 5).into());
		gem_pool -= Gem::Sapphire;
		assert_eq!(gem_pool, (0, 1, 3, 4, 5).into());
		gem_pool -= Gem::Emerald;
		assert_eq!(gem_pool, (0, 1, 2, 4, 5).into());
		gem_pool -= Gem::Ruby;
		assert_eq!(gem_pool, (0, 1, 2, 3, 5).into());
		gem_pool -= Gem::Onyx;
		assert_eq!(gem_pool, (0, 1, 2, 3, 4).into());
	}
}
