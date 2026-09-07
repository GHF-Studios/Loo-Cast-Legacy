//! Parsing stage: decimal/scientific literal -> canonical decimal parts.

use super::{CanonicalDecimal, DecimalWordNameError, MAX_FRACTIONAL_DIGITS, MAX_INTEGER_DIGITS};

/// Parses any supported decimal/scientific input into canonical parts.
pub(super) fn parse_decimal_literal(input: &str) -> Result<CanonicalDecimal, DecimalWordNameError> {
    let normalized = normalize_literal(input)?;
    let (negative, unsigned) = split_sign(normalized.as_str());
    let (mantissa, exponent) = split_scientific(unsigned)?;
    let (int_part, frac_part) = split_mantissa_parts(mantissa)?;

    let mut raw_digits = String::with_capacity(int_part.len() + frac_part.len());
    raw_digits.push_str(int_part);
    raw_digits.push_str(frac_part);

    if raw_digits.bytes().all(|b| b == b'0') {
        return Ok(CanonicalDecimal {
            negative: false,
            integer_digits: "0".to_string(),
            fractional_digits: String::new(),
        });
    }

    let decimal_point = int_part.len() as i64 + exponent as i64;
    let (integer_digits, fractional_digits) = if decimal_point <= 0 {
        build_parts_with_leading_fractional_zeros(raw_digits.as_str(), decimal_point)?
    } else if decimal_point >= raw_digits.len() as i64 {
        build_parts_with_trailing_integer_zeros(raw_digits.as_str(), decimal_point)?
    } else {
        let split_at = decimal_point as usize;
        (raw_digits[..split_at].to_string(), raw_digits[split_at..].to_string())
    };

    finalize_parts(negative, integer_digits, fractional_digits)
}

fn build_parts_with_leading_fractional_zeros(raw_digits: &str, decimal_point: i64) -> Result<(String, String), DecimalWordNameError> {
    let leading_zeros = (-decimal_point) as usize;
    if leading_zeros > MAX_FRACTIONAL_DIGITS {
        return Err(DecimalWordNameError::UnsupportedFractionalMagnitude {
            digits: leading_zeros,
            max_supported_digits: MAX_FRACTIONAL_DIGITS,
        });
    }

    let mut fractional_digits = String::with_capacity(leading_zeros + raw_digits.len());
    fractional_digits.push_str(&"0".repeat(leading_zeros));
    fractional_digits.push_str(raw_digits);
    Ok(("0".to_string(), fractional_digits))
}

fn build_parts_with_trailing_integer_zeros(raw_digits: &str, decimal_point: i64) -> Result<(String, String), DecimalWordNameError> {
    let zeros_to_append = (decimal_point as usize) - raw_digits.len();
    let total_digits = raw_digits.len() + zeros_to_append;
    if total_digits > MAX_INTEGER_DIGITS {
        return Err(DecimalWordNameError::UnsupportedIntegerMagnitude {
            digits: total_digits,
            max_supported_digits: MAX_INTEGER_DIGITS,
        });
    }

    let mut integer_digits = String::with_capacity(total_digits);
    integer_digits.push_str(raw_digits);
    integer_digits.push_str(&"0".repeat(zeros_to_append));
    Ok((integer_digits, String::new()))
}

fn finalize_parts(negative: bool, integer_digits: String, fractional_digits: String) -> Result<CanonicalDecimal, DecimalWordNameError> {
    let trimmed_integer = integer_digits.trim_start_matches('0');
    let integer_digits = if trimmed_integer.is_empty() {
        "0".to_string()
    } else {
        trimmed_integer.to_string()
    };

    let fractional_digits = fractional_digits.trim_end_matches('0').to_string();

    if fractional_digits.len() > MAX_FRACTIONAL_DIGITS {
        return Err(DecimalWordNameError::UnsupportedFractionalMagnitude {
            digits: fractional_digits.len(),
            max_supported_digits: MAX_FRACTIONAL_DIGITS,
        });
    }
    if integer_digits.len() > MAX_INTEGER_DIGITS {
        return Err(DecimalWordNameError::UnsupportedIntegerMagnitude {
            digits: integer_digits.len(),
            max_supported_digits: MAX_INTEGER_DIGITS,
        });
    }

    let is_zero = integer_digits == "0" && fractional_digits.is_empty();
    let negative = negative && !is_zero;
    validate_representable_range(negative, integer_digits.as_str())?;

    Ok(CanonicalDecimal {
        negative,
        integer_digits,
        fractional_digits,
    })
}

fn validate_representable_range(negative: bool, integer_digits: &str) -> Result<(), DecimalWordNameError> {
    if integer_digits.len() < MAX_INTEGER_DIGITS {
        return Ok(());
    }

    debug_assert_eq!(integer_digits.len(), MAX_INTEGER_DIGITS);
    let first = integer_digits.as_bytes()[0];
    let max_allowed_first = if negative { b'5' } else { b'4' };
    if first > max_allowed_first {
        return Err(DecimalWordNameError::OutOfRepresentableRange);
    }

    Ok(())
}

fn normalize_literal(input: &str) -> Result<String, DecimalWordNameError> {
    let raw = input.trim();
    if raw.is_empty() {
        return Err(DecimalWordNameError::EmptyInput);
    }

    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '−' | '﹣' | '－' => out.push('-'),
            '＋' => out.push('+'),
            ',' | '_' => {}
            c if c.is_whitespace() => {}
            _ => out.push(ch),
        }
    }

    if out.is_empty() {
        return Err(DecimalWordNameError::EmptyInput);
    }
    Ok(out)
}

fn split_sign(input: &str) -> (bool, &str) {
    if let Some(rest) = input.strip_prefix('+') {
        (false, rest)
    } else if let Some(rest) = input.strip_prefix('-') {
        (true, rest)
    } else {
        (false, input)
    }
}

fn split_scientific(input: &str) -> Result<(&str, i32), DecimalWordNameError> {
    let first_exp = input.find('e').or_else(|| input.find('E'));
    if let Some(idx) = first_exp {
        let mantissa = &input[..idx];
        let exp_text = &input[idx + 1..];

        if exp_text.contains('e') || exp_text.contains('E') {
            return Err(DecimalWordNameError::InvalidNumericLiteral(input.to_string()));
        }
        if mantissa.is_empty() || exp_text.is_empty() {
            return Err(DecimalWordNameError::InvalidNumericLiteral(input.to_string()));
        }

        let exponent = exp_text
            .parse::<i32>()
            .map_err(|_| DecimalWordNameError::InvalidNumericLiteral(input.to_string()))?;
        Ok((mantissa, exponent))
    } else {
        Ok((input, 0))
    }
}

fn split_mantissa_parts(mantissa: &str) -> Result<(&str, &str), DecimalWordNameError> {
    if mantissa.is_empty() {
        return Err(DecimalWordNameError::InvalidNumericLiteral(mantissa.to_string()));
    }

    if let Some(dot) = mantissa.find('.') {
        if mantissa[dot + 1..].contains('.') {
            return Err(DecimalWordNameError::InvalidNumericLiteral(mantissa.to_string()));
        }

        let int_part = if dot == 0 { "0" } else { &mantissa[..dot] };
        let frac_part = &mantissa[dot + 1..];
        if !int_part.bytes().all(|b| b.is_ascii_digit()) || !frac_part.bytes().all(|b| b.is_ascii_digit()) {
            return Err(DecimalWordNameError::InvalidNumericLiteral(mantissa.to_string()));
        }
        Ok((int_part, frac_part))
    } else {
        if !mantissa.bytes().all(|b| b.is_ascii_digit()) {
            return Err(DecimalWordNameError::InvalidNumericLiteral(mantissa.to_string()));
        }
        Ok((mantissa, ""))
    }
}
