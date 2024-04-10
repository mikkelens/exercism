use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
	Abundant,
	Perfect,
	Deficient
}

pub fn classify(num: u64) -> Option<Classification> {
	if num == 0 {
		None
	} else {
		let sum = (1..num).filter(|n| num % n == 0).sum();
		Some(match num.cmp(&sum) {
			Ordering::Less => Classification::Abundant,
			Ordering::Equal => Classification::Perfect,
			Ordering::Greater => Classification::Deficient
		})
	}
}
