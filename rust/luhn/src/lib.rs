/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	let valid_numbers = match code.len() {
		..=1 => None,
		_ => Some(code)
	}
	.map(|valid_str| to_numbers(valid_str.chars()))
	.flatten();

	valid_numbers.is_some_and(|numbers| {
		numbers.len() > 1 && double(numbers.into_iter()).sum::<u8>() % 10 == 0
	})
}

pub fn to_numbers(chars: impl Iterator<Item = char>) -> Option<Vec<u8>> {
	chars
		.into_iter()
		.filter(|c| !c.is_ascii_whitespace())
		.map(|c| c.to_digit(10).map(|num| num as u8))
		.collect::<Option<_>>()
}

pub fn double(
	nums: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>
) -> impl Iterator<Item = u8> {
	nums.rev()
		.enumerate()
		.map(|(index, mut a)| {
			assert!(a <= 9, "iter to double nums must not contain numbers >9");
			if index % 2 == 1 {
				a *= 2;
			}
			if a > 9 {
				a -= 9;
			}
			a
		})
		.rev()
}
