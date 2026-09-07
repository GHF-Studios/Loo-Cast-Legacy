/// Splits an optional leading sign from numeric text.
///
/// # Parameters
/// - `s`: Input text that may begin with `+` or `-`.
///
/// # Returns
/// - Tuple `(negative, body)` where:
///   - `negative` is `true` only when the input begins with `-`.
///   - `body` is the remaining text after removing one leading sign if present.
///
/// # Notes
/// - A leading `+` is accepted and treated as non-negative.
/// - This helper does not validate digits or numeric format.
pub fn split_leading_sign(s: &str) -> (bool, &str) {
    if let Some(rest) = s.strip_prefix('-') {
        (true, rest)
    } else if let Some(rest) = s.strip_prefix('+') {
        (false, rest)
    } else {
        (false, s)
    }
}

/// Splits scientific-notation text into mantissa and exponent parts.
///
/// # Parameters
/// - `s`: Input text expected to contain exactly one exponent marker (`e` or `E`).
///
/// # Returns
/// - Tuple `(mantissa, exponent)` where:
///   - `mantissa` is the substring before the exponent marker.
///   - `exponent` is the substring after the exponent marker.
///
/// # Panics
/// - Should panic when no exponent marker exists.
/// - Should panic when more than one exponent marker exists.
pub fn split_once_e_marker(s: &str) -> (&str, &str) {
    let idx = s.bytes().position(|b| b == b'e' || b == b'E').expect("missing exponent marker");
    let mantissa = &s[..idx];
    let exponent = &s[idx + 1..];
    assert!(!exponent.bytes().any(|b| b == b'e' || b == b'E'), "multiple exponent markers");
    (mantissa, exponent)
}

pub type Digit = u8;
pub type DigitBuffer<const N: usize> = [Digit; N];

/// Parses ASCII decimal digits into a preallocated digit buffer.
///
/// # Parameters
/// - `s`: Text expected to contain only ASCII digits (`0`..`9`).
/// - `out`: Destination buffer. Must have capacity for `s.len()` digits.
///
/// # Returns
/// - Written digit count (equal to `s.len()`).
///
/// # Panics
/// - Should panic when `s` contains any non-digit character.
/// - Should panic when `out` is too small.
pub fn parse_ascii_digits_into_buffer(s: &str, out: &mut [Digit]) -> usize {
    assert!(out.len() >= s.len(), "digit output buffer too small: need {}, got {}", s.len(), out.len(),);
    assert!(s.bytes().all(|b| b.is_ascii_digit()), "non-digit found");
    for (idx, b) in s.bytes().enumerate() {
        out[idx] = b - b'0';
    }
    s.len()
}

/// Normalizes a digit buffer by removing leading zeros while preserving one zero for all-zero input.
///
/// # Parameters
/// - `digits`: Digit buffer in big-endian order (most significant digit first).
/// - `len`: Amount of meaningful digits in `digits`.
///
/// # Behavior
/// - Returns the first index of the canonical view so that:
///   - either `digits[start] != 0`, or
///   - the value is represented by one zero digit at `len - 1`.
///
/// # Returns
/// - Start index of the trimmed view.
pub fn start_index_after_leading_zeros_keep_one(digits: &[Digit], len: usize) -> usize {
    assert!(len <= digits.len(), "trim length out of range");
    let mut start = 0_usize;
    while start + 1 < len && digits[start] == 0 {
        start += 1;
    }
    start
}

/// Normalizes a digit buffer by removing insignificant trailing zeros.
///
/// # Parameters
/// - `digits`: Digit buffer in big-endian order.
/// - `len`: Amount of meaningful digits in `digits`.
///
/// # Behavior
/// - Intended for contexts where trailing zeros do not affect value (for example fractional tails).
/// - Updates `len` in place to the trimmed end index.
pub fn trim_trailing_zeros_len(digits: &[Digit], len: &mut usize) {
    assert!(*len <= digits.len(), "trim length out of range");
    while *len > 0 && digits[*len - 1] == 0 {
        *len -= 1;
    }
}

/// Splits a scientific literal into sign, mantissa, and exponent text.
///
/// # Parameters
/// - `s`: Full scientific literal (for example `"1e3"`, `"-7.5E-2"`).
///
/// # Returns
/// - Tuple `(negative, mantissa, exponent_part)` where:
///   - `negative` is `true` when the literal begins with `-`.
///   - `mantissa` is the substring before the exponent marker.
///   - `exponent_part` is the substring after the exponent marker.
///
/// # Panics
/// - Should panic when input is empty.
/// - Should panic when sign-only input is provided.
/// - Should panic when exponent marker is missing.
/// - Should panic when mantissa or exponent text is missing.
pub fn split_scientific_literal_parts(s: &str) -> (bool, &str, &str) {
    assert!(!s.is_empty(), "invalid scientific literal: empty input");

    let (negative, body) = split_leading_sign(s);
    assert!(!body.is_empty(), "invalid scientific literal `{s}`: missing body");

    assert!(
        body.bytes().any(|b| b == b'e' || b == b'E'),
        "invalid scientific literal `{s}`: missing `e`/`E`",
    );
    let (mantissa, exp_part) = split_once_e_marker(body);

    assert!(!mantissa.is_empty(), "invalid scientific literal `{s}`: missing mantissa");
    assert!(!exp_part.is_empty(), "invalid scientific literal `{s}`: missing exponent");

    (negative, mantissa, exp_part)
}

/// Parses a signed scientific exponent segment into `i8`.
///
/// # Parameters
/// - `exp_part`: Exponent text segment (for example `"+12"`, `"-7"`, `"0"`).
/// - `whole`: Full scientific literal used for panic-message context.
///
/// # Returns
/// - Signed exponent value.
///
/// # Panics
/// - Should panic when exponent digits are missing.
/// - Should panic when exponent contains non-digit characters (after optional sign).
/// - Should panic on checked overflow while parsing or negating.
pub fn parse_signed_scientific_exponent_i8(exp_part: &str, whole: &str) -> i8 {
    let (exp_neg, exp_digits) = split_leading_sign(exp_part);
    assert!(!exp_digits.is_empty(), "invalid scientific literal `{whole}`: empty exponent digits",);
    assert!(
        exp_digits.bytes().all(|b| b.is_ascii_digit()),
        "invalid scientific literal `{whole}`: exponent must be signed digits",
    );

    let exp_abs = exp_digits
        .bytes()
        .map(|b| b - b'0')
        .try_fold(0_i8, |acc, d| acc.checked_mul(10).and_then(|v| v.checked_add(i8::try_from(d).unwrap())))
        .unwrap_or_else(|| panic!("invalid scientific literal `{whole}`: exponent too large"));

    if exp_neg {
        exp_abs
            .checked_neg()
            .unwrap_or_else(|| panic!("invalid scientific literal `{whole}`: exponent too large"))
    } else {
        exp_abs
    }
}

/// Splits and validates mantissa text into integer and fractional pieces.
///
/// # Parameters
/// - `mantissa`: Mantissa text segment (substring before exponent marker).
/// - `whole`: Full scientific literal used for panic-message context.
///
/// # Returns
/// - Tuple `(int_part, frac_part)` where:
///   - `int_part` is text before `.` (or entire mantissa when no `.` exists).
///   - `frac_part` is text after `.` (or empty when no `.` exists).
///
/// # Panics
/// - Should panic when mantissa contains characters other than digits and `.`.
/// - Should panic when mantissa contains multiple decimal points.
/// - Should panic when mantissa contains no digits.
pub fn split_scientific_mantissa_parts<'a>(mantissa: &'a str, whole: &str) -> (&'a str, &'a str) {
    assert!(
        mantissa.bytes().all(|b| b.is_ascii_digit() || b == b'.'),
        "invalid scientific literal `{whole}`: mantissa may only contain digits and `.`",
    );

    let (int_part, frac_part) = match mantissa.split_once('.') {
        Some((i, f)) => {
            assert!(!f.contains('.'), "invalid scientific literal `{whole}`: multiple `.` in mantissa",);
            (i, f)
        }
        None => (mantissa, ""),
    };

    assert!(
        !(int_part.is_empty() && frac_part.is_empty()),
        "invalid scientific literal `{whole}`: mantissa has no digits",
    );

    (int_part, frac_part)
}

/// Computes decimal-point placement for scientific mantissa digits.
///
/// # Parameters
/// - `coeff_len`: Amount of coefficient digits (mantissa digits without decimal point).
/// - `mantissa_frac_len`: Amount of mantissa fractional digits before exponent shift.
/// - `exponent`: Parsed signed exponent.
/// - `whole`: Full scientific literal used for panic-message context.
///
/// # Returns
/// - Signed decimal-point index relative to coefficient digits.
///
/// # Panics
/// - Should panic on checked overflow while adjusting exponent or placing decimal point.
pub fn scientific_decimal_point_index(coeff_len: usize, mantissa_frac_len: usize, exponent: i8, whole: &str) -> i8 {
    let coeff_len_i8 = i8::try_from(coeff_len).unwrap_or_else(|_| panic!("invalid scientific literal `{whole}`: decimal-point placement overflow"));
    let frac_len_i8 = i8::try_from(mantissa_frac_len).unwrap_or_else(|_| panic!("invalid scientific literal `{whole}`: exponent adjustment overflow"));

    let shift = exponent
        .checked_sub(frac_len_i8)
        .unwrap_or_else(|| panic!("invalid scientific literal `{whole}`: exponent adjustment overflow"));

    coeff_len_i8
        .checked_add(shift)
        .unwrap_or_else(|| panic!("invalid scientific literal `{whole}`: decimal-point placement overflow"))
}

/// Builds integer and fractional digits from coefficient digits and decimal-point index.
///
/// # Parameters
/// - `coeff`: Coefficient digits in big-endian order.
/// - `point`: Decimal-point index relative to `coeff`.
/// - `int_out`: Destination for variable-length integer digits.
/// - `frac_out`: Destination for variable-length fractional digits.
///
/// # Returns
/// - Tuple `(int_len, frac_len)` describing used prefixes in `int_out` and `frac_out`.
///
/// # Panics
/// - Panics when output buffers are too small for the requested decimal-point placement.
pub fn decimal_parts_from_coeff_and_point(coeff: &[u8], point: i8, int_out: &mut [u8], frac_out: &mut [u8]) -> (usize, usize) {
    assert!(!coeff.is_empty(), "coefficient must contain at least one digit");

    if point <= 0 {
        let left_pad = usize::try_from(-point).unwrap();
        let frac_len = left_pad + coeff.len();
        assert!(
            frac_len <= frac_out.len(),
            "fractional output buffer too small: need {}, got {}",
            frac_len,
            frac_out.len(),
        );
        frac_out[..left_pad].fill(0);
        frac_out[left_pad..frac_len].copy_from_slice(coeff);

        assert!(!int_out.is_empty(), "integer output buffer must not be empty");
        int_out[0] = 0;
        (1, frac_len)
    } else {
        let p = usize::try_from(point).unwrap();
        assert!(p <= int_out.len(), "integer output buffer too small: need {}, got {}", p, int_out.len(),);
        if p >= coeff.len() {
            int_out[..coeff.len()].copy_from_slice(coeff);
            int_out[coeff.len()..p].fill(0);
            (p, 0)
        } else {
            let frac_len = coeff.len() - p;
            assert!(
                frac_len <= frac_out.len(),
                "fractional output buffer too small: need {}, got {}",
                frac_len,
                frac_out.len(),
            );
            int_out[..p].copy_from_slice(&coeff[..p]);
            frac_out[..frac_len].copy_from_slice(&coeff[p..]);
            (p, frac_len)
        }
    }
}
