use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
	minutes: i32
}
impl Display for Clock {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let hours = self.minutes / 60;
		let minutes_within_hour = self.minutes % 60;
		write!(f, "{:0>2}:{:0>2}", hours, minutes_within_hour)
	}
}

impl Clock {
	pub fn new(hours: i32, minutes: i32) -> Self {
		Clock {
			minutes: (minutes + hours * 60).rem_euclid(24 * 60)
		}
	}

	pub fn add_minutes(&self, minutes: i32) -> Self {
		Clock::new(0, self.minutes + minutes)
	}
}
