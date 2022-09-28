use crate::gem_pool::{GemPool, GemPoolTuple};

#[derive(Debug, Clone, Copy)]
pub struct Aristocrat(GemPool);

impl From<GemPoolTuple> for Aristocrat {
	fn from(gem_pool_tuple: GemPoolTuple) -> Self {
		Self(gem_pool_tuple.into())
	}
}

impl std::ops::Deref for Aristocrat {
	type Target = GemPool;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_gem_pool_tuple() {
		let aristocrat: Aristocrat = (1, 2, 3, 4, 5).into();
		assert_eq!(aristocrat.diamonds(), 1);
		assert_eq!(aristocrat.sapphires(), 2);
		assert_eq!(aristocrat.emeralds(), 3);
		assert_eq!(aristocrat.rubies(), 4);
		assert_eq!(aristocrat.onyxes(), 5);
	}
}
