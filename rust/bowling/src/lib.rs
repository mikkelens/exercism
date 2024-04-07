use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
	NotEnoughPinsLeft,
	GameComplete
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum FrameState {
	RemainingPins(u16), // one shot left of two
	Finished(FrameResult)
}
impl FrameState {
	fn from_pins_knocks(pins: u16) -> Result<Self, Error> {
		Ok(match pins {
			0..=9 => FrameState::RemainingPins(10 - pins),
			10 => FrameState::Finished(FrameResult::Strike),
			_ => Err(Error::NotEnoughPinsLeft)?
		})
	}

	fn try_knock_pins(&mut self, pins: u16) -> Result<(), Error> {
		match self {
			FrameState::RemainingPins(remaining) if *remaining >= pins => {
				let previous_pins = 10 - *remaining;
				*self = match *remaining - pins {
					1.. => FrameState::Finished(FrameResult::Open(previous_pins, pins)),
					0 => FrameState::Finished(FrameResult::Spare(previous_pins, pins))
				};
				Ok(())
			},
			FrameState::RemainingPins(_) | FrameState::Finished(_) => Err(Error::NotEnoughPinsLeft)
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(unused)]
enum FrameResult {
	Strike,          // always 10, first throw
	Spare(u16, u16), // we need to count the amount for each throw
	Open(u16, u16)
}

pub struct BowlingGame {
	frames:     Vec<FrameState>,
	fill_balls: (Option<FillBall>, Option<FillBall>)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct FillBall(Option<u16>);

impl BowlingGame {
	pub fn new() -> Self {
		BowlingGame {
			frames:     Vec::new(),
			fill_balls: (None, None)
		}
	}

	/// "Record that {pins} pins have been scored"
	/// Can roll if below/at 10th frame, OR if you have fill balls for
	/// spare/strike on last frame. Maximum fill balls is 2 received from strike
	/// on last (10th) frame, meaning a whole 11th frame extra.
	pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
		// assert!((0..=10).contains(&pins)); // handled by error?
		let last_len = self.frames.len();
		eprint!("rolling with {} existing... ", last_len);
		match (self.frames.last().clone(), last_len, &mut self.fill_balls) {
			(Some(FrameState::Finished(_)), 10, (Some(FillBall(fill_ball)), _))
				if *fill_ball == None =>
			{
				eprintln!("Matched 1st fill ball!");
				if !(0..=10).contains(&pins) {
					Err(Error::NotEnoughPinsLeft)?;
				}
				*fill_ball = Some(pins);
			},
			(Some(FrameState::Finished(_)), 10, (first_fill, Some(FillBall(fill_ball))))
				if *fill_ball == None =>
			{
				eprintln!("Matched 2nd fill ball!");
				let Some(FillBall(Some(first_fill))) = first_fill else {
					unreachable!("this is filled first always if the second is matched");
				};
				let first_fill = *first_fill;
				if (!(0..=10).contains(&pins)) || (first_fill != 10 && (10 - first_fill) < pins) {
					eprintln!("Second fill ball failed. First fill pins: {}", first_fill);
					Err(Error::NotEnoughPinsLeft)?;
				}
				*fill_ball = Some(pins);
			},
			(Some(FrameState::RemainingPins(_)), previous_len, _) => {
				eprintln!("trying to knock down the last pins...");
				let previous = self.frames.last_mut().expect("already matched Some()");
				previous.try_knock_pins(pins)?; // never adds another frame
				match previous {
					FrameState::RemainingPins(_) => unreachable!("just completed roll"),
					FrameState::Finished(result) => {
						eprintln!("Result: {:?}", result);
						self.fill_balls = match (previous_len, &result) {
							(_, FrameResult::Strike) => {
								unreachable!("Cannot finish remaining and get a strike!")
							},
							// (10, FrameResult::Strike) => {
							// 	,
							(10, FrameResult::Spare(_, _)) => {
								eprintln!("Marked up one fill ball with spare!",);
								(
									self.fill_balls.0.or(Some(FillBall(None))),
									self.fill_balls.0
								)
							},
							_ => self.fill_balls
						}
					}
				}
			},
			(None, _, _) | (Some(FrameState::Finished(_)), ..=9, _) => {
				let new_frame = FrameState::from_pins_knocks(pins)?;
				eprintln!("Rolled into new frame: {:?}", new_frame);
				self.fill_balls = match (&new_frame, last_len) {
					(FrameState::Finished(FrameResult::Strike), 9) => {
						eprintln!("Marked *two* fill balls with strike (last frame)!");
						(Some(FillBall(None)), Some(FillBall(None)))
					},
					(FrameState::Finished(FrameResult::Strike), 8) => {
						eprintln!("Marked *one* fill balls with strike (pre-last frame)!");
						(Some(FillBall(None)), self.fill_balls.1)
					},
					_ => self.fill_balls
				};
				self.frames.push(new_frame); // initialization rolls
			},
			(Some(FrameState::Finished(_)), 10.., (val1, val2)) => {
				eprintln!(
					"ERROR: Already finished, and fill balls are {:?} and {:?}?",
					val1, val2
				);
				Err(Error::GameComplete)?
			}
		};
		Ok(())
	}

	/// "Return the score if the game is complete, or None if not."
	pub fn score(&self) -> Option<u16> {
		eprintln!("\ncalculating score...\n");
		match self.frames.len() {
			0..=9 => None,
			10 => {
				let mut total = 0;
				let mut carry_multipliers: VecDeque<u16> = VecDeque::new();
				for frame in self.frames.iter() {
					match frame {
						// if a frame is unfinished, the game is not done
						FrameState::RemainingPins(_) => None?,
						FrameState::Finished(frame_result) => {
							eprintln!("- {:?}", frame_result);
							let first_carry_multiplier = carry_multipliers.pop_front().unwrap_or(0);
							eprintln!("existing 1st carry: {}", first_carry_multiplier);
							match frame_result {
								FrameResult::Strike => {
									total += 10 + (10 * first_carry_multiplier);

									// update/set carry multipliers ahead
									if let Some(carry_multiplier) = carry_multipliers.get_mut(0) {
										// after this
										eprintln!("added 1st strike multiplier to existing");
										*carry_multiplier += 1;
									} else {
										eprintln!("created 1st strike multiplier");
										carry_multipliers.push_back(1);
									}
									if let Some(carry_multiplier) = carry_multipliers.get_mut(1) {
										// after that
										eprintln!("added 2nd strike multiplier to existing");
										*carry_multiplier += 1;
									} else {
										eprintln!("created 2nd strike multiplier");
										carry_multipliers.push_back(1);
									}
								},
								FrameResult::Spare(first, second) => {
									total += first + (first * first_carry_multiplier);
									let second_carry_multiplier =
										carry_multipliers.pop_front().unwrap_or(0);
									total += second + (second * second_carry_multiplier);

									// update/set carry multiplier ahead
									if let Some(carry_multiplier) = carry_multipliers.get_mut(0) {
										eprintln!("added spare multiplier to existing");
										*carry_multiplier += 1;
									} else {
										eprintln!("created spare multiplier");
										carry_multipliers.push_back(1);
									}
								},
								FrameResult::Open(first, second) => {
									total += first + (first * first_carry_multiplier);
									let second_carry_multiplier =
										carry_multipliers.pop_front().unwrap_or(0);
									total += second + (second * second_carry_multiplier);
								}
							};
						}
					}
				}
				eprintln!("All frames have been counted, counting any potential fill balls...");
				if let Some(FillBall(first)) = self.fill_balls.0 {
					let first_ball = first?;
					total += first_ball
						* carry_multipliers
							.pop_front()
							.expect("should have a ball fill carry left?");
					eprintln!("Fill ball counted: {}", first_ball);
					if let Some(FillBall(second)) = self.fill_balls.1 {
						let second_ball = second?;
						total += second_ball
							* carry_multipliers
								.pop_front()
								.expect("should have another last ball fill carry left?");
						eprintln!("Fill ball counted: {}", second_ball);
					}
				} else {
					debug_assert_eq!(self.fill_balls.1, None);
				}
				match carry_multipliers.len() {
					1.. => None, // no more frames, but fill balls are needed to end
					0 => Some(total)
				}
			},
			_ => unreachable!("should not be able to get >10 frames (not including fill balls)")
		}
	}
}
