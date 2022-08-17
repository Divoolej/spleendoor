pub type GemPoolTuple = (u8, u8, u8, u8, u8);

pub struct GemPool {
	diamond: u8,
	saphire: u8,
	emerald: u8,
	ruby: u8,
	onyx: u8,
}

impl From<GemPoolTuple> for GemPool {
	fn from((diamond, saphire, emerald, ruby, onyx): GemPoolTuple) -> Self {
		Self {
			diamond,
			saphire,
			emerald,
			ruby,
			onyx,
		}
	}
}
