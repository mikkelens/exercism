use std::iter;

pub fn is_armstrong_number(num: u32) -> bool {
	let all_digits: Vec<_> =
		iter::successors(Some(num), |&n| if n >= 10 { Some(n / 10) } else { None })
			.map(|n| n % 10)
			.collect();
	let digit_num = all_digits.len() as u32;
	all_digits
		.iter()
		.filter_map(|digit| digit.checked_pow(digit_num))
		// try_fold also handles case with zero/zero digits
		.try_fold(0u32, |acc, n| acc.checked_add(n))
		.map(|sum| sum == num) // check sum equal to number
		.unwrap_or(false) // if we can't check sum (was too large) it was wrong
}
