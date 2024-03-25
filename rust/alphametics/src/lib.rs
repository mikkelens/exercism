use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
	let (a, rest) = input.split_once(" + ")?;
	let (b, c) = rest.split_once(" == ")?;

	let set = a
		.chars()
		.chain(b.chars())
		.chain(c.chars())
		.collect::<HashSet<_>>();

	assert!(set.len() <= 10); // all must be unique, too many characters means not enough decimal digits

	try_find_valid(
		HashMap::new(),
		set.into_iter().collect(),
		&(
			&a.chars().collect::<Vec<_>>()[..],
			&b.chars().collect::<Vec<_>>()[..],
			&c.chars().collect::<Vec<_>>()[..]
		)
	)
	.filter(|inner| {
		// ensure no leading zeroes
		*inner.get(&a.chars().next().unwrap()).unwrap() != 0
			&& *inner.get(&b.chars().next().unwrap()).unwrap() != 0
			&& *inner.get(&c.chars().next().unwrap()).unwrap() != 0
	})
}

fn try_find_valid(
	decided: HashMap<char, u8>,
	undecided: Vec<char>,
	(a, b, c): &(&[char], &[char], &[char])
) -> Option<HashMap<char, u8>> {
	assert!(decided.len() <= 10); // all must be unique
	if undecided.is_empty() {
		// try to see if out path was correct
		let value = |iter: &[char]| {
			iter.iter()
				.rev()
				.enumerate()
				.map(|(i, e)| *decided.get(e).unwrap() as u32 * 10_u32.pow(i as u32))
				.sum::<u32>()
		};
		let a = value(a);
		let b = value(b);
		let c = value(c);
		if a + b == c {
			Some(decided)
		} else {
			None
		}
	} else {
		// look through all valid paths (DFS)
		(0..undecided.len()).find_map(|next_index| {
			// abitrary letter
			let mut next_undecided = undecided.clone();
			let next = next_undecided.remove(next_index);

			(0..=9)
				.filter(|val| !decided.values().contains(val))
				.find_map(|valid_digit| {
					// abitrary (valid) digit
					let mut next_decided = decided.clone();
					let previous = next_decided.insert(next, valid_digit);
					assert_eq!(previous, None);
					try_find_valid(next_decided, next_undecided.clone(), &(a, b, c))
				})
		})
	}
}
