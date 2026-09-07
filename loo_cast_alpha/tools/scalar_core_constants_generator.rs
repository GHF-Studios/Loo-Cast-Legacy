#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! num-bigint = "0.4.6"
//! ```

use num_bigint::{BigInt, Sign};
use std::fs;
use std::path::{Path, PathBuf};

const INT_LEN: usize = 36;
const FRAC_INTERNAL_LEN: usize = 44;
const CONST_PREFIX: &str = "SCALAR";
const INPUT_FILE: &str = "scalar_constants.txt";

#[derive(Debug)]
struct ScalarConversion {
    name: String,
    balanced_negative: bool,
    radix_position: usize,
    int_balanced: [i8; INT_LEN],
    frac_balanced_internal: [i8; FRAC_INTERNAL_LEN],
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    if std::env::args().len() != 1 {
        return Err(
            "no flags supported: run with `rust-script scalar_core_constants_generator.rs`"
                .to_string(),
        );
    }

    let tool_dir = detect_tool_dir()?;
    let input_path = tool_dir.join(INPUT_FILE);
    let output_path = tool_dir.join("generated/scalar_constants_seed.rs");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("failed to create {:?}: {e}", parent))?;
    }

    let entries = load_named_literals(&input_path)?;
    let mut conversions = Vec::with_capacity(entries.len());
    for (name, literal) in entries {
        let converted = convert_entry(&name, &literal)
            .map_err(|e| format!("{e} (name={name}, literal={literal})"))?;
        conversions.push(converted);
    }

    let rust_text = emit_rust(&conversions, CONST_PREFIX)?;
    fs::write(&output_path, rust_text).map_err(|e| format!("failed writing {:?}: {e}", output_path))?;

    println!(
        "Wrote {} proven constants to {}",
        conversions.len(),
        output_path.display()
    );
    Ok(())
}

fn detect_tool_dir() -> Result<PathBuf, String> {
    let exe_dir = PathBuf::from(file!())
        .parent()
        .unwrap()
        .to_path_buf();

    if exe_dir.join(INPUT_FILE).is_file() {
        return Ok(exe_dir);
    }

    Err(format!("could not find input file ({INPUT_FILE})"))
}

fn load_named_literals(path: &Path) -> Result<Vec<(String, String)>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("failed to read {:?}: {e}", path))?;
    let mut out = Vec::new();

    for (line_no, raw_line) in text.lines().enumerate() {
        let mut line = raw_line;
        if let Some(idx) = line.find('#') {
            line = &line[..idx];
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parsed = if let Some(eq) = line.find('=') {
            (line[..eq].trim(), line[eq + 1..].trim())
        } else if let Some(colon) = line.find(':') {
            (line[..colon].trim(), line[colon + 1..].trim())
        } else {
            let auto_name = format!("item_{}", line_no + 1);
            out.push((auto_name, line.to_string()));
            continue;
        };

        if parsed.0.is_empty() {
            return Err(format!("line {}: empty name", line_no + 1));
        }
        if parsed.1.is_empty() {
            return Err(format!("line {}: empty value", line_no + 1));
        }

        out.push((parsed.0.to_string(), parsed.1.to_string()));
    }

    Ok(out)
}

fn convert_entry(name: &str, literal: &str) -> Result<ScalarConversion, String> {
    let scaled_expected = parse_scaled_literal(literal)?;
    let (int_bal, frac_bal, balanced_negative, radix_position) =
        balanced_from_scaled(&scaled_expected)?;

    let scaled_actual = scaled_from_balanced(&int_bal, &frac_bal);
    if scaled_actual != scaled_expected {
        return Err(format!(
            "proof mismatch for {name}: expected {}, got {}",
            scaled_expected, scaled_actual
        ));
    }

    Ok(ScalarConversion {
        name: name.to_string(),
        balanced_negative,
        radix_position,
        int_balanced: int_bal,
        frac_balanced_internal: frac_bal,
    })
}

fn parse_scaled_literal(raw: &str) -> Result<BigInt, String> {
    let normalized = normalize_numeric_literal(raw)?;
    let (negative, body) = split_leading_sign(&normalized);
    let (mantissa, exponent) = split_scientific(&body)?;
    let (int_part, frac_part) = split_decimal_parts(mantissa)?;

    if int_part.len() > INT_LEN {
        return Err(format!(
            "integer part exceeds {} digits: {}",
            INT_LEN, int_part
        ));
    }

    let coeff_str = {
        let mut s = String::with_capacity(int_part.len() + frac_part.len());
        s.push_str(int_part);
        s.push_str(frac_part);
        let trimmed = s.trim_start_matches('0');
        if trimmed.is_empty() {
            "0".to_string()
        } else {
            trimmed.to_string()
        }
    };

    let coeff = parse_bigint_base10(&coeff_str)?;
    if coeff.sign() == Sign::NoSign {
        return Ok(BigInt::from(0u8));
    }

    let frac_len = frac_part.len() as i32;
    let shift = exponent - frac_len + FRAC_INTERNAL_LEN as i32;

    let mut scaled_abs = if shift >= 0 {
        coeff * pow10_bigint(shift as usize)
    } else {
        let divisor = pow10_bigint((-shift) as usize);
        let q = &coeff / &divisor;
        let r = &coeff % &divisor;
        let mut rounded = q;
        if (&r * 2u8) >= divisor {
            rounded += 1u8;
        }
        rounded
    };

    if negative && scaled_abs.sign() != Sign::NoSign {
        scaled_abs = -scaled_abs;
    }
    Ok(scaled_abs)
}

fn normalize_numeric_literal(raw: &str) -> Result<String, String> {
    let text = raw.trim();
    if text.is_empty() {
        return Err("empty numeric literal".to_string());
    }
    if text.contains("...") || text.contains('…') {
        return Err("truncated numeric literal (ellipsis found)".to_string());
    }

    // Normalize signs and remove separators.
    let mut normalized = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '−' | '﹣' | '－' => normalized.push('-'),
            '＋' => normalized.push('+'),
            ',' | '_' => {}
            c if c.is_whitespace() => {}
            _ => normalized.push(ch),
        }
    }

    // Drop Wolfram precision marks: e.g. 3.14`50 or 3.14`*^10.
    let mut precision_clean = String::with_capacity(normalized.len());
    let chars: Vec<char> = normalized.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '`' {
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            continue;
        }
        precision_clean.push(chars[i]);
        i += 1;
    }

    // Normalize Wolfram scientific forms.
    let mut sci = precision_clean.replace("*^", "e");
    sci = sci.replace("×10^", "e");
    sci = sci.replace("x10^", "e");
    sci = sci.replace("X10^", "e");
    sci = sci.replace("·10^", "e");

    validate_numeric_syntax(&sci)?;
    Ok(sci)
}

fn validate_numeric_syntax(s: &str) -> Result<(), String> {
    if s.is_empty() {
        return Err("empty numeric literal after normalization".to_string());
    }

    let mut saw_exp = false;
    let mut saw_dot = false;
    let mut saw_digit = false;
    let mut exp_digits = 0usize;

    let mut chars = s.chars().peekable();
    if matches!(chars.peek(), Some('+') | Some('-')) {
        chars.next();
    }

    while let Some(ch) = chars.next() {
        match ch {
            '0'..='9' => {
                saw_digit = true;
                if saw_exp {
                    exp_digits += 1;
                }
            }
            '.' => {
                if saw_exp || saw_dot {
                    return Err(format!("invalid numeric literal: {s}"));
                }
                saw_dot = true;
            }
            'e' | 'E' => {
                if saw_exp || !saw_digit {
                    return Err(format!("invalid numeric literal: {s}"));
                }
                saw_exp = true;
                exp_digits = 0;
                if matches!(chars.peek(), Some('+') | Some('-')) {
                    chars.next();
                }
            }
            _ => return Err(format!("invalid numeric literal: {s}")),
        }
    }

    if !saw_digit {
        return Err(format!("invalid numeric literal: {s}"));
    }
    if saw_exp && exp_digits == 0 {
        return Err(format!("invalid numeric literal: {s}"));
    }

    Ok(())
}

fn split_leading_sign(s: &str) -> (bool, &str) {
    if let Some(rest) = s.strip_prefix('+') {
        (false, rest)
    } else if let Some(rest) = s.strip_prefix('-') {
        (true, rest)
    } else {
        (false, s)
    }
}

fn split_scientific(s: &str) -> Result<(&str, i32), String> {
    if let Some(idx) = s.find('e').or_else(|| s.find('E')) {
        let mantissa = &s[..idx];
        let exp_str = &s[idx + 1..];
        let exponent = exp_str
            .parse::<i32>()
            .map_err(|_| format!("invalid scientific exponent: {exp_str}"))?;
        Ok((mantissa, exponent))
    } else {
        Ok((s, 0))
    }
}

fn split_decimal_parts(mantissa: &str) -> Result<(&str, &str), String> {
    if let Some(idx) = mantissa.find('.') {
        let int_part = if idx == 0 { "0" } else { &mantissa[..idx] };
        let frac_part = &mantissa[idx + 1..];
        if !int_part.chars().all(|c| c.is_ascii_digit()) || !frac_part.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!("invalid decimal mantissa: {mantissa}"));
        }
        Ok((int_part, frac_part))
    } else {
        if !mantissa.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!("invalid decimal mantissa: {mantissa}"));
        }
        Ok((mantissa, ""))
    }
}

fn parse_bigint_base10(s: &str) -> Result<BigInt, String> {
    BigInt::parse_bytes(s.as_bytes(), 10).ok_or_else(|| format!("failed to parse bigint: {s}"))
}

fn pow10_bigint(exp: usize) -> BigInt {
    let mut out = BigInt::from(1u8);
    for _ in 0..exp {
        out *= 10u8;
    }
    out
}

fn balanced_from_scaled(
    scaled: &BigInt,
) -> Result<([i8; INT_LEN], [i8; FRAC_INTERNAL_LEN], bool, usize), String> {
    let ten = BigInt::from(10u8);
    let five = BigInt::from(5u8);

    let mut n = scaled.clone();
    let mut rev_digits: Vec<i8> = Vec::new();
    while n != BigInt::from(0u8) {
        let x = &n + &five;
        let rem = ((&x % &ten) + &ten) % &ten;
        let rem_i8 = rem
            .to_string()
            .parse::<i8>()
            .map_err(|_| "internal: failed converting remainder to i8".to_string())?;
        let digit = rem_i8 - 5;
        rev_digits.push(digit);
        n = (n - BigInt::from(digit)) / 10u8;
    }

    if rev_digits.is_empty() {
        rev_digits.push(0);
    }

    let mut int_bal = [0i8; INT_LEN];
    let mut frac_bal = [0i8; FRAC_INTERNAL_LEN];
    for (idx, digit) in rev_digits.iter().copied().enumerate() {
        let power = idx as isize - FRAC_INTERNAL_LEN as isize;
        if power >= 0 {
            let power_usize = power as usize;
            if power_usize >= INT_LEN {
                return Err(format!(
                    "integer part exceeds {} digits after balanced conversion",
                    INT_LEN
                ));
            }
            let int_idx = INT_LEN - 1 - power_usize;
            int_bal[int_idx] = digit;
        } else {
            let frac_idx = (-power - 1) as usize;
            if frac_idx >= FRAC_INTERNAL_LEN {
                return Err(format!(
                    "fractional part exceeds {} digits after balanced conversion",
                    FRAC_INTERNAL_LEN
                ));
            }
            frac_bal[frac_idx] = digit;
        }
    }

    let is_zero = int_bal.iter().all(|d| *d == 0) && frac_bal.iter().all(|d| *d == 0);
    let balanced_negative = scaled.sign() == Sign::Minus && !is_zero;

    let mut radix_position = INT_LEN - 1;
    let mut linear_idx = 0usize;
    for d in int_bal {
        if d != 0 {
            radix_position = linear_idx;
        }
        linear_idx += 1;
    }
    for d in frac_bal {
        if d != 0 {
            radix_position = linear_idx;
        }
        linear_idx += 1;
    }

    Ok((int_bal, frac_bal, balanced_negative, radix_position))
}

fn scaled_from_balanced(int_bal: &[i8; INT_LEN], frac_bal: &[i8; FRAC_INTERNAL_LEN]) -> BigInt {
    let mut value = BigInt::from(0u8);
    for &digit in int_bal {
        value *= 10u8;
        value += BigInt::from(digit);
    }
    for &digit in frac_bal {
        value *= 10u8;
        value += BigInt::from(digit);
    }
    value
}

fn emit_rust(conversions: &[ScalarConversion], const_prefix: &str) -> Result<String, String> {
    let prefix = make_const_ident(const_prefix);
    let mut out = String::new();
    let mut seen = std::collections::HashSet::<String>::new();

    out.push_str("// @generated by scalar_core_constants_generator.rs\n");
    out.push_str("// This file is generated. Do not edit manually.\n\n");
    out.push_str(&format!(
        "pub type ScalarCoreConst = ([i8; {INT_LEN}], [i8; {FRAC_INTERNAL_LEN}], bool, usize);\n\n"
    ));

    for conv in conversions {
        let base = make_const_ident(&conv.name);
        let const_name = if prefix.is_empty() {
            base
        } else {
            format!("{prefix}_{base}")
        };
        if !seen.insert(const_name.clone()) {
            return Err(format!("duplicate constant name after normalization: {const_name}"));
        }

        out.push_str(&format!("pub const {const_name}: ScalarCoreConst = ("));
        out.push_str(&format!(
            "{}, {}, {}, {});\n",
            format_i8_array(&conv.int_balanced),
            format_i8_array(&conv.frac_balanced_internal),
            if conv.balanced_negative { "true" } else { "false" },
            conv.radix_position
        ));
    }

    Ok(out)
}

fn make_const_ident(raw: &str) -> String {
    let mut out = String::new();
    let mut prev_underscore = false;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_uppercase());
            prev_underscore = false;
        } else if !prev_underscore {
            out.push('_');
            prev_underscore = true;
        }
    }
    out = out.trim_matches('_').to_string();
    if out.is_empty() {
        out = "VALUE".to_string();
    }
    if out
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        out = format!("N_{out}");
    }
    out
}

fn format_i8_array<const N: usize>(values: &[i8; N]) -> String {
    let mut out = String::from("[");
    for (i, v) in values.iter().enumerate() {
        if i != 0 {
            out.push_str(", ");
        }
        out.push_str(&v.to_string());
    }
    out.push(']');
    out
}
