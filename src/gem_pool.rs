pub type GemPoolTuple = (u8, u8, u8, u8, u8);

pub struct GemPool {
	diamond: u8,
	sapphire: u8,
	emerald: u8,
	ruby: u8,
	onyx: u8,
}

impl GemPool {
	pub fn new(initial_amount: u8) -> Self {
		(initial_amount, initial_amount, initial_amount, initial_amount, initial_amount).into()
	}
}

impl From<GemPoolTuple> for GemPool {
	fn from((diamond, sapphire, emerald, ruby, onyx): GemPoolTuple) -> Self {
		Self {
			diamond,
			sapphire,
			emerald,
			ruby,
			onyx,
		}
	}
}