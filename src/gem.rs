use std::ops::Mul;

use crate::gem_pool::GemPool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gem {
	Diamond,
	Sapphire,
	Emerald,
	Ruby,
	Onyx,
}

impl Mul<u8> for Gem {
	type Output = GemPool;

	fn mul(self, rhs: u8) -> Self::Output {
		match self {
			Gem::Diamond => GemPool {
				diamonds: rhs,
				..Default::default()
			},
			Gem::Sapphire => GemPool {
				sapphires: rhs,
				..Default::default()
			},
			Gem::Emerald => GemPool {
				emeralds: rhs,
				..Default::default()
			},
			Gem::Ruby => GemPool {
				rubies: rhs,
				..Default::default()
			},
			Gem::Onyx => GemPool {
				onyxes: rhs,
				..Default::default()
			},
		}
	}
}

impl Mul<Gem> for u8 {
	type Output = GemPool;

	fn mul(self, rhs: Gem) -> Self::Output {
		match rhs {
			Gem::Diamond => GemPool {
				diamonds: self,
				..Default::default()
			},
			Gem::Sapphire => GemPool {
				sapphires: self,
				..Default::default()
			},
			Gem::Emerald => GemPool {
				emeralds: self,
				..Default::default()
			},
			Gem::Ruby => GemPool {
				rubies: self,
				..Default::default()
			},
			Gem::Onyx => GemPool {
				onyxes: self,
				..Default::default()
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mul_u8_for_gem() {
		assert_eq!(
			Gem::Diamond * 1,
			GemPool {
				diamonds: 1,
				..Default::default()
			}
		);
		assert_eq!(
			Gem::Sapphire * 2,
			GemPool {
				sapphires: 2,
				..Default::default()
			}
		);
		assert_eq!(
			Gem::Emerald * 3,
			GemPool {
				emeralds: 3,
				..Default::default()
			}
		);
		assert_eq!(
			Gem::Ruby * 4,
			GemPool {
				rubies: 4,
				..Default::default()
			}
		);
		assert_eq!(
			Gem::Onyx * 5,
			GemPool {
				onyxes: 5,
				..Default::default()
			}
		);
	}

	#[test]
	fn test_mul_gem_for_u8() {
		assert_eq!(
			5 * Gem::Diamond,
			GemPool {
				diamonds: 5,
				..Default::default()
			}
		);
		assert_eq!(
			4 * Gem::Sapphire,
			GemPool {
				sapphires: 4,
				..Default::default()
			}
		);
		assert_eq!(
			3 * Gem::Emerald,
			GemPool {
				emeralds: 3,
				..Default::default()
			}
		);
		assert_eq!(
			2 * Gem::Ruby,
			GemPool {
				rubies: 2,
				..Default::default()
			}
		);
		assert_eq!(
			1 * Gem::Onyx,
			GemPool {
				onyxes: 1,
				..Default::default()
			}
		);
	}
}
