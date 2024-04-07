#[derive(Eq, PartialEq, Copy, Clone)]
enum Field {
	Empty,
	AdjacentToBombs(u8),
	Bomb
}
impl From<&char> for Field {
	fn from(c: &char) -> Self {
		match c {
			&' ' => Field::Empty,
			&'*' => Field::Bomb,
			_ => unimplemented!("not a char that should be found here")
		}
	}
}
impl From<&Field> for char {
	fn from(field: &Field) -> Self {
		match field {
			Field::Empty => ' ',
			Field::AdjacentToBombs(num) => match num {
				// I hate that this is necessary to display a digit properly
				0..=8 => char::from_digit(*num as u32, 10).unwrap(),
				_ => unreachable!("not possible to have >8 neighbors")
			},
			Field::Bomb => '*'
		}
	}
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
	eprintln!("Asked to annotate:\n{}", minefield.join("\n"));
	let minefield: Vec<Vec<Field>> = minefield
		.into_iter()
		.map(|s| s.chars().map(|c| (&c).into()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let minefield = &minefield
		.iter()
		.map(|fields| &fields[..])
		.collect::<Vec<_>>()[..];
	minefield
		.iter()
		.enumerate()
		.map(|(y, fields)| {
			fields
				.into_iter()
				.enumerate()
				.map(|(x, &field)| {
					char::from(&if field == Field::Empty {
						field_with_mine_count(minefield, (x, y))
					} else {
						field
					})
				})
				.collect::<String>()
		})
		.collect()
}

fn field_with_mine_count(minefield: &[&[Field]], (origin_x, origin_y): (usize, usize)) -> Field {
	eprintln!(
		"Counting adjacent for empty field {:?}",
		(origin_x, origin_y)
	);
	let adjacent = ((origin_y.saturating_sub(1))
		..=(origin_y.saturating_add(1).min(minefield.len() - 1)))
		.into_iter()
		.flat_map(|y| {
			((origin_x.saturating_sub(1))
				..=(origin_x.saturating_add(1).min(minefield[y].len() - 1)))
				.into_iter()
				.filter(move |&x| (x, y) != (origin_x, origin_y)) // skip self
				.inspect(move |&x| eprintln!("...checking {:?}...", (x, y)))
				.map(move |x| minefield[y][x])
				.filter(|field| field == &Field::Bomb)
				.inspect(|_| eprintln!("...bomb found!"))
		})
		.count();
	eprintln!("--> found {} for {:?}", adjacent, (origin_x, origin_y));
	if adjacent > 0 {
		Field::AdjacentToBombs(adjacent as u8)
	} else {
		Field::Empty
	}
}
