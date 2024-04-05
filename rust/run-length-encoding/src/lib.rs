use std::fmt::Write;

use itertools::Itertools;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .dedup_with_count()
        .fold(String::new(), |mut s, (n, c)| {
            let _ = match n {
                1 => write!(s, "{}", c),
                _ => write!(s, "{}{}", n, c),
            };
            s
        })
}

pub fn decode(source: &str) -> String {
    let mut groups: Vec<(char, usize)> = vec![];
    let mut next_num_chars = vec![];
    for c in source.chars() {
        if c.is_numeric() {
            next_num_chars.push(c);
        } else {
            groups.push((
                c,
                next_num_chars
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap_or(1),
            ));
            next_num_chars.clear();
        }
    }

    groups
        .into_iter()
        .flat_map(|(c, n)| std::iter::repeat(c).take(n))
        .collect()
}
