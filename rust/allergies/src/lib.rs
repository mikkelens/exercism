use enumflags2::{bitflags, BitFlags};

pub struct Allergies(BitFlags<Allergen>);

/// This problem provides us with values that can be interpreted as bitflags
/// (values as powers of two). I import a crate that automatically implements
/// bitflags for this enum, and also gives us an iterator over each flagged enum
/// variant. Maybe not make my own bitflag implementation is cheating, but I
/// think this is generally what I would do in an actual project anyway.
#[bitflags]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
	// I could specify the bits/number, but the default (1 -> 128) is what I want
	Eggs,
	Peanuts,
	Shellfish,
	Strawberries,
	Tomatoes,
	Chocolate,
	Pollen,
	Cats
}

impl Allergies {
	pub fn new(score: u32) -> Self {
		Self(
			BitFlags::from_bits(
				// allergens with a value over or equal 256 are ignored/thrown away in this cast
				score as u8
			)
			.expect("all u8 bits are used in Allergen bitflag")
		)
	}

	pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
		self.0.contains(*allergen)
	}

	pub fn allergies(&self) -> Vec<Allergen> {
		self.0.iter().collect()
	}
}
