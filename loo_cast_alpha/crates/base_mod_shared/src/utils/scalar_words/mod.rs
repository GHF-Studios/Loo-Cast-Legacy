//! Decimal/scientific literal to English-word conversion for scalar naming.
//!
//! Public API:
//! - [`decimal_string_to_word_tokens`]: canonical token stream
//! - [`decimal_string_to_snake_case`]: snake_case name derived from the same tokens

mod error;
mod parse;
#[cfg(test)]
mod tests;
mod words;

pub use error::DecimalWordNameError;

/// Maximum supported integer digits for the scalar model (`10^35` scale band).
const MAX_INTEGER_DIGITS: usize = 36;
/// Maximum supported fractional digits for the scalar model (`10^-44` scale band).
const MAX_FRACTIONAL_DIGITS: usize = 44;

/// Canonical parsed decimal value used by the formatter stage.
#[derive(Debug, Clone)]
struct CanonicalDecimal {
    negative: bool,
    integer_digits: String,
    fractional_digits: String,
}

/// Converts a decimal/scientific literal into canonical word tokens.
///
/// Examples:
/// - `0.000000000000000001` => `["one", "over", "one", "quintillion"]`
/// - `6000` => `["six", "thousand"]`
pub fn decimal_string_to_word_tokens(input: &str) -> Result<Vec<String>, DecimalWordNameError> {
    let value = parse::parse_decimal_literal(input)?;
    let tokens = words::tokens_from_canonical_decimal(&value)?;
    Ok(tokens.into_iter().map(str::to_string).collect())
}

/// Converts a decimal/scientific literal into `snake_case`.
pub fn decimal_string_to_snake_case(input: &str) -> Result<String, DecimalWordNameError> {
    Ok(decimal_string_to_word_tokens(input)?.join("_"))
}
