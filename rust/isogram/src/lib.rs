use itertools::Itertools;

pub fn check(candidate: &str) -> bool {
	candidate
		.chars()
        .map(|c| c.to_ascii_lowercase())
		.filter(|&c| c != '-' && c != ' ')
		.all_unique()
}
