pub type GemPoolTuple = (u8, u8, u8, u8, u8);

#[derive(Debug, Clone)]
pub struct GemPool {
	diamonds: u8,
	sapphires: u8,
	emeralds: u8,
	rubies: u8,
	onyxes: u8,
}

impl GemPool {
	pub fn new(initial_amount: u8) -> Self {
		(initial_amount, initial_amount, initial_amount, initial_amount, initial_amount).into()
	}

	pub fn total(&self) -> u8 {
		self.diamonds + self.sapphires + self.emeralds + self.rubies + self.onyxes
	}

	pub fn diamonds(&self) -> u8 { self.diamonds }
	pub fn sapphires(&self) -> u8 { self.sapphires }
	pub fn emeralds(&self) -> u8 { self.emeralds }
	pub fn rubies(&self) -> u8 { self.rubies }
	pub fn onyxes(&self) -> u8 { self.onyxes }
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
	fn test_from_gem_pool_tuple() {
		let gem_pool_tuple = (1, 2, 3, 4, 5);
		let gem_pool: GemPool = gem_pool_tuple.into();
		assert_eq!(gem_pool.diamonds, 1);
		assert_eq!(gem_pool.sapphires, 2);
		assert_eq!(gem_pool.emeralds, 3);
		assert_eq!(gem_pool.rubies, 4);
		assert_eq!(gem_pool.onyxes, 5);
	}
}