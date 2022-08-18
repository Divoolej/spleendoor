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