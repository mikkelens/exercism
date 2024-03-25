use std::collections::HashSet;

use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
	let word_lowercase = word.to_lowercase(); // case-insensitive
	let word_letters = word_lowercase.chars().counts();
	possible_anagrams
		.iter()
		.filter(|candidate| {
			let other_word_lowercase = candidate.to_lowercase(); // also case-insensitive
			other_word_lowercase.chars().counts() == word_letters // has the same letters
				&& other_word_lowercase != word_lowercase // not the same word
		})
		.map(|&s| s)
		.collect()
}
