pub fn factors(n: u64) -> Vec<u64> {
	let mut prime_factors = vec![];
	let mut rest = n;
	'factor_creation: for factor in 2.. {
		while rest % factor == 0 {
			rest /= factor;
			prime_factors.push(factor);
		}
		if rest == 1 {
			break 'factor_creation;
		}
	}
	prime_factors
}
