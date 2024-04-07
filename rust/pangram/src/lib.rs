use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
	const ALPHABET_LOWERCASE: [char; 26] = [
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
		's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
	];
	let set = HashSet::from(ALPHABET_LOWERCASE);
	sentence
		.chars()
		.map(|c| c.to_ascii_lowercase())
		.collect::<HashSet<char>>()
		.is_superset(&set)
}
