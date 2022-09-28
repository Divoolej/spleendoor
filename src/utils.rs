use rand::{distributions::Distribution, thread_rng, Rng};

static SEED_CHARACTERS: [char; 25] = [
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'W', 'X',
	'Y', 'Z',
];

struct SeedCharacters;

impl Distribution<char> for SeedCharacters {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
		use rand::seq::SliceRandom;

		*SEED_CHARACTERS.choose(rng).unwrap()
	}
}

pub fn generate_seed() -> String {
	thread_rng().sample_iter(SeedCharacters).take(6).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_generate_seed() {
		let seed = generate_seed();
		assert_eq!(seed.len(), 6);
		for c in seed.chars() {
			assert!(SEED_CHARACTERS.contains(&c));
		}
	}
}
