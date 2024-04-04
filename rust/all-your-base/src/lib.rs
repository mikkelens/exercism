#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still
/// pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the
///    output must be `[0]`. However, your function must be able to process
///    input with leading 0 digits.
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // ensure valid bases (base of 1 means the only digit is zero)
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    // if there are no numbers, output is zero
    if number.is_empty() {
        return Ok(vec![0]);
    }

    // we can convert input with "from" base to a value total (rust integer) with
    // arithmetic using input/from base. 321 in base 4 to value:
    // 1*3^0 + 2*3^1 + 3*3^2 = 1 + 6 + 27 = 34 (notice this does not become 321)
    let total = number
        .into_iter()
        .rev() // number bases are assumed to start from lowest->highest, going right->left
        .enumerate()
        .try_fold(0, |acc, (offset, &digit)| {
            if digit >= from_base {
                // bases are 1 higher than the highest representable digit
                Err(Error::InvalidDigit(digit))
            } else {
                Ok(acc + digit * from_base.pow(offset as u32))
            }
        })?;

    // find the offset that is the most significant required 0-indexed place
    let most_significant_offset = 'test: {
        for offset in 0.. {
            let place_value = to_base.pow(offset);
            if total / place_value < to_base {
                break 'test offset; // won't need another digit to represent number
            }
        }
        unreachable!("some place will be the last")
    };

    // take from the top of total, and each digit will be the amount taken
    let (_zero, output) = (0..=most_significant_offset).rev().fold(
        (total, Vec::new()),
        |(mut reducing_total, mut output), offset| {
            let place_value = to_base.pow(offset);
            let place_digit = reducing_total / place_value;
            output.push(place_digit); // cannot be zero for first in iter
            reducing_total -= place_value * place_digit;
            (reducing_total, output)
        },
    );
    debug_assert_eq!(_zero, 0);
    Ok(output)
}
