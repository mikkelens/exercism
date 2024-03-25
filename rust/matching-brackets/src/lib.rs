#[derive(Debug)]
struct Bracket {
	opening: bool,
	variant: BracketVariants
}
#[derive(Debug, Eq, PartialEq)]
enum BracketVariants {
	Parentheses, // ()
	Brackets,    // []
	Braces       // {}
}
impl TryFrom<char> for Bracket {
	type Error = String;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		Ok(match c {
			'(' => Bracket {
				opening: true,
				variant: BracketVariants::Parentheses
			},
			')' => Bracket {
				opening: false,
				variant: BracketVariants::Parentheses
			},
			'[' => Bracket {
				opening: true,
				variant: BracketVariants::Brackets
			},
			']' => Bracket {
				opening: false,
				variant: BracketVariants::Brackets
			},
			'{' => Bracket {
				opening: true,
				variant: BracketVariants::Braces
			},
			'}' => Bracket {
				opening: false,
				variant: BracketVariants::Braces
			},
			_ => Err("char is not a bracket".to_string())?
		})
	}
}

pub fn brackets_are_balanced(string: &str) -> bool {
	string
		.chars()
		.filter_map(|c| Bracket::try_from(c).ok()) // ignore non-brackets
		.try_fold(Vec::new(), |mut acc, bracket| {
			if bracket.opening {
				// opening brackets are always accepted
				acc.push(bracket);
				Some(acc)
			} else if acc
				.last()
				.map(|last| last.variant == bracket.variant)
				.unwrap_or(false /* not any opening brackets */)
			{
				// closing bracket matches top of stack/previous
				let _ = acc.pop();
				Some(acc)
			} else {
				// closing bracket did not match previous opening bracket
				None
			}
		})
		.map(|end_state| end_state.is_empty()) // no excess brackets
		.unwrap_or(false) // found illegal motion earlier
}
