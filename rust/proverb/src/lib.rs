use itertools::Itertools;

pub fn build_proverb(list: &[&str]) -> String {
	match list.first() {
		None => String::new(),
		Some(first) => list
			.iter()
			.tuple_windows()
			.map(|(a, b)| format!("For want of a {} the {} was lost.\n", a, b))
			.chain(std::iter::once(format!(
				"And all for the want of a {}.",
				first
			)))
			.collect()
	}
}
