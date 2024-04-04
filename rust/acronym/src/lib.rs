pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '-') // words seperated by hyphens or whitespace
        .filter_map(|word| {
            // ensure word is capitalized
            let mut chars = word.chars().filter(|c| c.is_ascii_alphabetic());
            chars.next().map(|first| {
                std::iter::once(first.to_ascii_uppercase())
                    .chain(chars)
                    .collect::<String>()
            })
        })
        .flat_map(|capitalized_word| {
            // find sub-words (spans of capital letters, can be just the whole word),
            // then take the first character of each
            capitalized_word
                .split(|c: char| c.is_ascii_lowercase())
                .filter_map(|sub_word| sub_word.chars().next())
                .collect::<Vec<char>>()
                .into_iter()
        })
        .collect::<String>()
}
