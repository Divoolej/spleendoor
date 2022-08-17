use crate::gem_pool::{GemPool, GemPoolTuple};

pub struct Aristocrat(GemPool);

impl From<GemPoolTuple> for Aristocrat {
	fn from(gem_pool_tuple: GemPoolTuple) -> Self {
		Self(gem_pool_tuple.into())
	}
}