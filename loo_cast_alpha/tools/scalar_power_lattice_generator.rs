#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! base_mod_shared = { path = "../crates/base_mod_shared" }
//! ```

use base_mod_shared::utils::scalar_words::decimal_string_to_snake_case;
use std::fs::{File, create_dir_all};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

const MIN_EXP: i32 = -44;
const MAX_EXP: i32 = 35;
const MAX_SAFE_TOP_MANTISSA: u32 = 4; // at 10^35, 5..9 overflows after balanced carry
const MAX_LITERAL: &str = "444444444444444444444444444444444444.44444444444444444444444444444444444444444444";
const FIVE_HUNDRED_DECILLION_LITERAL: &str = "500000000000000000000000000000000000";
const MIN_LITERAL: &str = "-555555555555555555555555555555555555.55555555555555555555555555555555555555555555";

fn main() -> io::Result<()> {
    println!("generating power lattice...");

    let exe_dir = PathBuf::from(file!()).parent().unwrap().to_path_buf();
    let generated_dir = exe_dir.join("generated");
    create_dir_all(&generated_dir)?;
    let output_path = generated_dir.join("power_lattice.txt");

    let file = File::create(&output_path)?;
    let mut out = BufWriter::new(file);

    let mut emitted = 0usize;
    let mut all_entries: Vec<(String, String)> = Vec::new();

    for exp in MIN_EXP..=MAX_EXP {
        let bucket_entries = build_bucket_entries(exp)?;
        all_entries.extend(bucket_entries);
    }

    writeln!(out, "maximum={MAX_LITERAL}")?;
    emitted += 1;

    writeln!(out, "minimum={MIN_LITERAL}")?;
    emitted += 1;

    for (name, value) in all_entries.iter().rev() {
        writeln!(out, "{name}={value}")?;
        emitted += 1;
    }

    writeln!(out, "zero=0")?;
    emitted += 1;

    writeln!(out)?;

    for (name, value) in all_entries {
        writeln!(out, "negative_{name}=-{value}")?;
        emitted += 1;
    }

    writeln!(
        out,
        "negative_five_hundred_decillion=-{FIVE_HUNDRED_DECILLION_LITERAL}"
    )?;
    emitted += 1;

    out.flush()?;
    println!("done");
    println!("entries emitted: {}", emitted);
    println!("output: {}", output_path.display());
    Ok(())
}

fn build_bucket_entries(exp: i32) -> io::Result<Vec<(String, String)>> {
    let max_mantissa = if exp == MAX_EXP {
        MAX_SAFE_TOP_MANTISSA
    } else {
        9
    };

    let mut entries: Vec<(String, String)> = Vec::with_capacity(max_mantissa as usize);
    for mantissa in 1u32..=max_mantissa {
        let value = render_value(mantissa, exp);
        let name = decimal_string_to_snake_case(&value).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("failed to convert value `{value}` to name: {error}"),
            )
        })?;
        entries.push((name, value));
    }

    Ok(entries)
}

fn render_value(mantissa: u32, exp: i32) -> String {
    if exp >= 0 {
        let mut s = mantissa.to_string();
        s.push_str(&"0".repeat(exp as usize));
        s
    } else {
        let zeros = (-exp - 1) as usize;
        format!("0.{}{}", "0".repeat(zeros), mantissa)
    }
}
