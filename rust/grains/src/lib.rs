pub fn square(s: u32) -> u64 {
	// handles starting-at-one offset (from 2) with no underflow elegantly, but
	// overflows at 64 2u64.pow(s) / 2

	// does not overflow, and underflow is restricted by assertion
	assert!((1..=64).contains(&s), "Square must be between 1 and 64");
	2u64.pow(s - 1)
}

pub fn total() -> u64 {
	(1..=64).map(|s| square(s)).sum()
}
