//! Formatting stage: canonical decimal parts -> canonical word tokens.

use super::{CanonicalDecimal, DecimalWordNameError, MAX_FRACTIONAL_DIGITS, MAX_INTEGER_DIGITS};

/// Builds canonical English tokens from canonical parsed decimal parts.
pub(super) fn tokens_from_canonical_decimal(value: &CanonicalDecimal) -> Result<Vec<&'static str>, DecimalWordNameError> {
    let mut tokens: Vec<&'static str> = Vec::new();

    if value.integer_digits == "0" && value.fractional_digits.is_empty() {
        tokens.push("zero");
        return Ok(tokens);
    }

    if value.negative {
        tokens.push("negative");
    }

    if value.fractional_digits.is_empty() {
        tokens.extend(integer_tokens_from_non_zero_digits(value.integer_digits.as_str())?);
    } else if value.integer_digits == "0" {
        tokens.extend(fraction_tokens(value.fractional_digits.as_str())?);
    } else {
        tokens.extend(integer_tokens_from_non_zero_digits(value.integer_digits.as_str())?);
        tokens.push("and");
        tokens.extend(fraction_tokens(value.fractional_digits.as_str())?);
    }

    Ok(tokens)
}

fn fraction_tokens(frac_digits: &str) -> Result<Vec<&'static str>, DecimalWordNameError> {
    let numerator_digits = frac_digits.trim_start_matches('0');
    let numerator_digits = if numerator_digits.is_empty() { "0" } else { numerator_digits };

    let mut tokens = integer_tokens_from_non_zero_digits(numerator_digits)?;
    tokens.push("over");
    tokens.extend(denominator_tokens(frac_digits.len())?);
    Ok(tokens)
}

fn denominator_tokens(exp: usize) -> Result<Vec<&'static str>, DecimalWordNameError> {
    match exp {
        0 => Err(DecimalWordNameError::InvalidNumericLiteral("missing fractional exponent".to_string())),
        1 => Ok(vec!["ten"]),
        2 => Ok(vec!["one", "hundred"]),
        _ => {
            if exp > MAX_FRACTIONAL_DIGITS {
                return Err(DecimalWordNameError::UnsupportedFractionalMagnitude {
                    digits: exp,
                    max_supported_digits: MAX_FRACTIONAL_DIGITS,
                });
            }

            let rem = exp % 3;
            let group_exp = exp - rem;
            let group = magnitude_word(group_exp)?;
            let tokens = match rem {
                0 => vec!["one", group],
                1 => vec!["ten", group],
                2 => vec!["one", "hundred", group],
                _ => unreachable!(),
            };
            Ok(tokens)
        }
    }
}

fn integer_tokens_from_non_zero_digits(digits: &str) -> Result<Vec<&'static str>, DecimalWordNameError> {
    let digits = digits.trim_start_matches('0');
    if digits.is_empty() {
        return Ok(vec!["zero"]);
    }
    if digits.len() > MAX_INTEGER_DIGITS {
        return Err(DecimalWordNameError::UnsupportedIntegerMagnitude {
            digits: digits.len(),
            max_supported_digits: MAX_INTEGER_DIGITS,
        });
    }

    let remainder = digits.len() % 3;
    let mut padded = String::with_capacity(digits.len() + (3 - remainder) % 3);
    if remainder != 0 {
        padded.push_str(&"0".repeat(3 - remainder));
    }
    padded.push_str(digits);

    let total_chunks = padded.len() / 3;
    let mut out = Vec::new();

    for (idx, chunk) in padded.as_bytes().chunks(3).enumerate() {
        let chunk_value = ((chunk[0] - b'0') as u16) * 100 + ((chunk[1] - b'0') as u16) * 10 + (chunk[2] - b'0') as u16;
        if chunk_value == 0 {
            continue;
        }

        out.extend(chunk_tokens(chunk_value));

        let group_exp = (total_chunks - idx - 1) * 3;
        if group_exp > 0 {
            out.push(magnitude_word(group_exp)?);
        }
    }

    if out.is_empty() {
        out.push("zero");
    }
    Ok(out)
}

fn chunk_tokens(mut value: u16) -> Vec<&'static str> {
    debug_assert!((1..=999).contains(&value));
    let mut out = Vec::new();

    if value >= 100 {
        out.push(digit_word((value / 100) as u8));
        out.push("hundred");
        value %= 100;
    }

    if value >= 10 {
        out.push(tens_word((value / 10) as u8));
        if value % 10 != 0 {
            out.push(digit_word((value % 10) as u8));
        }
    } else if value > 0 {
        out.push(digit_word(value as u8));
    }

    out
}

fn magnitude_word(exp: usize) -> Result<&'static str, DecimalWordNameError> {
    let word = match exp {
        3 => "thousand",
        6 => "million",
        9 => "billion",
        12 => "trillion",
        15 => "quadrillion",
        18 => "quintillion",
        21 => "sextillion",
        24 => "septillion",
        27 => "octillion",
        30 => "nonillion",
        33 => "decillion",
        36 => "undecillion",
        39 => "duodecillion",
        42 => "tredecillion",
        _ => {
            return Err(DecimalWordNameError::UnsupportedIntegerMagnitude {
                digits: exp + 1,
                max_supported_digits: MAX_INTEGER_DIGITS,
            });
        }
    };
    Ok(word)
}

fn digit_word(value: u8) -> &'static str {
    match value {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => unreachable!(),
    }
}

fn tens_word(value: u8) -> &'static str {
    match value {
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => unreachable!(),
    }
}
