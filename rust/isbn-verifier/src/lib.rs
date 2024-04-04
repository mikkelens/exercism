/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let non_hyphen: String = isbn.chars().filter(|&c| c != '-').collect();
    non_hyphen.len() == 10
        && non_hyphen
            .chars()
            .rev()
            .enumerate()
            .try_fold(0, |acc, (i, c)| {
                match c {
                    'X' => {
                        // X may only be last (index is from right to left)
                        if i == 0 {
                            Some(10)
                        } else {
                            None
                        }
                    }
                    d => d.to_digit(10),
                }
                .map(|n| (i as u32 + 1) * n + acc)
            })
            .is_some_and(|sum| sum % 11 == 0)
}
