use itertools::Itertools;

#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
	pub fn new(scores: &'a [u32]) -> Self {
		HighScores(scores)
	}

	pub fn scores(&self) -> &'a [u32] {
		self.0
	}

	pub fn latest(&self) -> Option<u32> {
		self.0.last().copied()
	}

	pub fn personal_best(&self) -> Option<u32> {
		self.0.iter().max().copied()
	}

	pub fn personal_top_three(&self) -> Vec<u32> {
		self.0
			.iter()
			.sorted_unstable_by(|a, b| b.cmp(a)) // rev sort
			.take(3) // takes top three
			.copied()
			.collect()
	}
}
