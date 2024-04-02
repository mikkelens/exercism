use itertools::Itertools;

/// `Palindrome` is a newtype which only exists when the contained value is a
/// palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this
/// is called a "newtype", and its use is often referred to as the "newtype
/// pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
	/// Create a `Palindrome` only if `value` is in fact a palindrome when
	/// represented in base ten. Otherwise, `None`.
	pub fn new(value: u64) -> Option<Palindrome> {
		if Self::is_palindrome(value) {
			Some(Palindrome(value))
		} else {
			None
		}
	}

	fn is_palindrome(value: u64) -> bool {
		let reverse = value
			.to_string()
			.chars()
			.rev()
			.collect::<String>()
			.parse::<u64>()
			.unwrap();
		reverse == value
	}

	/// Get the value of this palindrome.
	pub fn into_inner(self) -> u64 {
		self.0
	}
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
	let forwards = min..=max;
	let backwards = forwards.clone().rev();
	fn find_product(iter: impl Iterator<Item = u64> + Clone) -> Option<Palindrome> {
		iter.clone()
			.cartesian_product(iter)
			.map(|(a, b)| a * b)
			.find_map(Palindrome::new)
	}

	let first = find_product(forwards);
	let last = find_product(backwards);
	first.zip(last)
}
