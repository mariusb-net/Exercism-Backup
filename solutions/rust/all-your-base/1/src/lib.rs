#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
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
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let decimal = to_decimal(number, from_base)?;
    if decimal.is_empty() {
        return Ok(vec![0]);
    }   
    to_bases(decimal, to_base)
}

fn to_decimal(number: &[u32], base: u32) -> Result<Vec<u32>, Error> {
    if base < 2 {
        return Err(Error::InvalidInputBase);
    }

    let mut decimal_value: u128 = 0;
    for &digit in number {
        if digit >= base {
            return Err(Error::InvalidDigit(digit));
        }
        decimal_value = decimal_value
            .checked_mul(base as u128)
            .and_then(|v| v.checked_add(digit as u128))
            .ok_or(Error::InvalidDigit(digit))?;
    }

    Ok(vec![decimal_value as u32])
}

fn to_bases(decimal: Vec<u32>, base: u32) -> Result<Vec<u32>, Error> {
    if base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut value = decimal[0] as u128;
    if value == 0 {
        return Ok(vec![0]);
    }

    let mut result = Vec::new();
    while value > 0 {
        result.push((value % base as u128) as u32);
        value /= base as u128;
    }
    result.reverse();
    Ok(result)
}