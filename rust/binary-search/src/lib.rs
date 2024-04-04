use std::cmp::Ordering;

pub fn find<T: Eq + Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();
    match array.len() {
        0 => None, // prevent index-out-of-bounds (given empty array)
        1 => {
            // prevent searching forever
            if array[0] == key {
                Some(0)
            } else {
                None
            }
        }
        len => {
            let middle_index = len / 2;
            let middle_key = &array[middle_index];
            match key.cmp(&middle_key) {
                Ordering::Equal => Some(middle_index),
                Ordering::Less => find(&array[..middle_index], key),
                Ordering::Greater => {
                    find(&array[middle_index..], key).map(|index| index + middle_index)
                }
            }
        }
    }
}
