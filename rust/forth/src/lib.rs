extern crate core;

use std::{collections::HashMap, str::FromStr};

pub type FValue = i32;
pub type FResult = Result<(), FError>;

#[derive(Debug)]
pub struct Forth {
	state:          FState,
	custom_symbols: HashMap<String, Vec<FOperation>>,
	stack:          Vec<FValue>
}

#[derive(Debug, PartialEq, Eq)]
enum FState {
	Stack,
	DefiningCustomWord(Option<(String, Vec<FOperation>)>)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum FWord {
	StateChange(FStateChange),
	Operation(FOperation)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum FOperation {
	Number(FValue),
	Arithmetic(FArithmetic),
	Manipulation(FStackManip),
	NonKeyword(String)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum FStateChange {
	BeginDef,
	EndDefinition
}

impl FromStr for FWord {
	type Err = FError;

	/// Parse substrings as keywords or symbols.
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			// state change
			":" => FWord::StateChange(FStateChange::BeginDef),
			";" => FWord::StateChange(FStateChange::EndDefinition),
			// all operations
			op => FWord::Operation(match op {
				// integer arithmetic
				"+" => FOperation::Arithmetic(FArithmetic::Plus),
				"-" => FOperation::Arithmetic(FArithmetic::Minus),
				"*" => FOperation::Arithmetic(FArithmetic::Multiply),
				"/" => FOperation::Arithmetic(FArithmetic::Divide),
				// stack manipulations
				"dup" => FOperation::Manipulation(FStackManip::DUP),
				"drop" => FOperation::Manipulation(FStackManip::DROP),
				"swap" => FOperation::Manipulation(FStackManip::SWAP),
				"over" => FOperation::Manipulation(FStackManip::OVER),
				num => num
					.parse::<FValue>()
					.map(FOperation::Number)
					.unwrap_or(FOperation::NonKeyword(num.to_string()))
			})
		})
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum FArithmetic {
	Plus,
	Minus,
	Multiply,
	Divide
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum FStackManip {
	DUP,
	DROP,
	SWAP,
	OVER
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FError {
	DivisionByZero,
	StackUnderflow,
	UnknownWord,
	InvalidWord
}

impl Forth {
	pub fn new() -> Forth {
		Forth {
			state:          FState::Stack,
			custom_symbols: HashMap::new(),
			stack:          vec![]
		}
	}

	pub fn stack(&self) -> &[FValue] {
		self.stack.as_slice()
	}

	/// result of evaluating '{input}
	pub fn eval(&mut self, input: &str) -> FResult {
		eprintln!("Started evaluation.");
		for instruction in input.split_whitespace() {
			eprintln!("Evaluating new instruction/word.");
			dbg!(&self.state);
			dbg!(&self.stack);
			dbg!(&self.custom_symbols);
			dbg!(&instruction);
			let word = instruction.parse::<FWord>()?;
			dbg!(&word);
			match &mut self.state {
				FState::Stack => match word {
					FWord::StateChange(change) => match change {
						FStateChange::BeginDef => {
							self.state = FState::DefiningCustomWord(None); // begin definition
						},
						FStateChange::EndDefinition => {
							return Err(FError::InvalidWord); // cannot end def from eval
						}
					},
					FWord::Operation(op) => self.stack_operation(op)?
				},
				FState::DefiningCustomWord(def_data) => match word {
					// start/end definition context
					FWord::StateChange(change) => match (change, def_data) {
						(FStateChange::BeginDef, _) | (FStateChange::EndDefinition, None) => {
							return Err(FError::InvalidWord); // cannot start or stop def like this
						},
						(FStateChange::EndDefinition, Some((name, def_operations))) => {
							let _prev = self
								.custom_symbols
								.insert(name.clone(), def_operations.clone());
							// don't use prev for anything
							self.state = FState::Stack;
						}
					},
					FWord::Operation(op) => match (op, def_data) {
						// create definition name
						(FOperation::NonKeyword(available), def_data @ None) => {
							// initialize def with name
							*def_data = Some((available, vec![]));
							// we still aren't done with the definition
						},
						(_keyword, None) => {
							return Err(FError::InvalidWord); // cannot use keyword as def name
						},
						(keyword, Some((_, def_operations))) => {
							def_operations.push(keyword); // add to the end
							  // use of undefined words inside definition is
							  // allowed
						}
					}
				}
			}

			eprintln!();
		}
		Ok(())
	}

	fn stack_operation(&mut self, op: FOperation) -> FResult {
		let stack = &mut self.stack;
		match op {
			FOperation::Number(n) => {
				stack.push(n);
			},
			FOperation::Arithmetic(op) => {
				// `b` is the right hand side, at the top of the stack
				let b = stack.pop().ok_or(FError::StackUnderflow)?;
				let a = stack.pop().ok_or(FError::StackUnderflow)?;
				let res = match op {
					FArithmetic::Plus => a + b,
					FArithmetic::Minus => a - b,
					FArithmetic::Multiply => a * b,
					FArithmetic::Divide => {
						if b == 0 {
							Err(FError::DivisionByZero)?
						} else {
							a / b
						}
					},
				};
				stack.push(res);
			},
			FOperation::Manipulation(manip) => {
				match manip {
					FStackManip::DUP => {
						let top = stack.last().ok_or(FError::StackUnderflow)?;
						stack.push(*top); // copy ("dup") it on top
					},
					FStackManip::DROP => {
						let _ = stack.pop().ok_or(FError::StackUnderflow)?;
						// drop value
					},
					FStackManip::SWAP => {
						let above = stack.pop().ok_or(FError::StackUnderflow)?;
						let below = stack.pop().ok_or(FError::StackUnderflow)?;
						stack.push(above);
						stack.push(below); // below is now on top of stack
					},
					FStackManip::OVER => {
						let below = stack
							.last_chunk::<2>()
							.ok_or(FError::StackUnderflow)?
							.first()
							.unwrap();
						stack.push(*below); // add value below top value onto the top of stack
					}
				}
			},
			FOperation::NonKeyword(symbol_name) => {
				let ops = {
					let Some(def_words) = self.custom_symbols.get(&symbol_name) else {
						return Err(FError::UnknownWord);
					};
					def_words.clone()
				};
				for def_op in ops.into_iter() {
					// recursively evaluate stack using operations from function "body" (value)
					self.stack_operation(def_op)?;
				}
			}
		}
		Ok(())
	}
}
