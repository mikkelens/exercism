use std::cmp::Ordering;

use itertools::{FoldWhile, Itertools};

/// `Palindrome` is a newtype which only exists when the contained value is a
/// palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this
/// is called a "newtype", and its use is often referred to as the "newtype
/// pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if Self::is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }

    /// Find out if a number is a palindrome by reversing the bytes of its string representation and comparing it to the original.
    /// Furthermore, we only need to compare each half of the string bytes, since a palindrome is mirrored in the middle.
    fn is_palindrome(value: u64) -> bool {
        let s = value.to_string();
        let half = s.len() / 2; // lower bounded, but used for indexing (offset 1 higher)
        let first = s.bytes().take(half);
        let second = s.bytes().rev().take(half);
        first.eq(second) // if first half is equal to second half in reverse, it is a palindrome
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let forwards = min..=max;
    let backwards = forwards.clone().rev();

    /// In order to avoid allocating every combination at once, we instead create an iterator that does this incrementally.
    /// By giving this function the right input we can find the min or max palindrome in a range-based iterator,
    /// without having to explore every possible combination.
    fn best_palindrome_by_order(
        iter_oriented: impl Iterator<Item = u64> + Clone, // ranges are only reversible as iterators
        ord_target: Ordering,
    ) -> Option<Palindrome> {
        let ideal = iter_oriented.clone().next()?; // component that informs early end
        iter_oriented
            .clone()
            .cartesian_product(iter_oriented) // the core of this approach
            .fold_while(None, |best_so_far, (a, b)| {
                if best_so_far
                    .is_some_and(|inner: Palindrome| inner.0.cmp(&(a * ideal)) == ord_target)
                {
                    // cannot yield a product with superior ordering from this point, optimal next is `a * ideal`
                    FoldWhile::Done(best_so_far)
                } else {
                    let product = a * b;
                    let superior_found =
                        !best_so_far.is_some_and(|inner| product.cmp(&inner.0) != ord_target);
                    FoldWhile::Continue(
                        if let Some(palindrome) =
                            superior_found.then(|| Palindrome::new(product)).flatten()
                        {
                            Some(palindrome) // superior product is palindrome, replacing
                        } else {
                            best_so_far // new product does not have superior ordering OR not a palindrome
                        },
                    )
                }
            })
            .into_inner()
    }

    let min = best_palindrome_by_order(forwards, Ordering::Less);
    let max = best_palindrome_by_order(backwards, Ordering::Greater);
    // if min and max are the same, we have "wasted time" looking for the same value twice,
    // but this is insignificant because this may only happen on small inputs, and cannot generally be predicted

    min.zip(max) // None-precedence of Option-zip is irrelevant because min and max are always the same variant
}