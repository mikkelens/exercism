use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
	h.iter()
		.flat_map(|(point, characters)| characters.iter().map(|c| (c.to_ascii_lowercase(), *point)))
		.collect()
}
