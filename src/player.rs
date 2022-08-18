use crate::token_pool::TokenPool;
use crate::card::Card;

pub type PlayerIndex = usize;

#[derive(Debug, Clone)]
pub struct Player {
	tokens: TokenPool,
	cards: Vec<Card>,
	reserved_cards: Vec<Card>,
}

impl Player {
	pub fn new() -> Self {
		Self {
			tokens: TokenPool::empty(),
			cards: Vec::new(),
			reserved_cards: Vec::new(),
		}
	}
}