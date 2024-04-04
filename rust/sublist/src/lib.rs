#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_array: &[T], second_array: &[T]) -> Comparison {
    if (first_array.is_empty() && second_array.is_empty()) || first_array == second_array {
        Comparison::Equal
    } else if second_array.is_empty()
        || first_array
            .windows(second_array.len())
            .any(|first_window| first_window == second_array)
    {
        Comparison::Superlist
    } else if first_array.is_empty()
        || second_array
            .windows(first_array.len())
            .any(|second_window| second_window == first_array)
    {
        Comparison::Sublist
    } else {
        Comparison::Unequal // no other pattern
    }
}
