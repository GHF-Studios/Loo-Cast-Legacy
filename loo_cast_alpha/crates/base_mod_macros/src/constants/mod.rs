use proc_macro::TokenStream;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Canonical range-sample constants module paths.
const RANGE_SAMPLE_MODULES: &[&str] = &[
    "positive_decillion",
    "positive_nonillion",
    "positive_octillion",
    "positive_septillion",
    "positive_sextillion",
    "positive_quintillion",
    "positive_quadrillion",
    "positive_trillion",
    "positive_billion",
    "positive_million",
    "positive_thousand",
    "positive_peanut",
    "positive_one_over_peanut",
    "positive_one_over_thousand",
    "positive_one_over_million",
    "positive_one_over_billion",
    "positive_one_over_trillion",
    "positive_one_over_quadrillion",
    "positive_one_over_quintillion",
    "positive_one_over_sextillion",
    "positive_one_over_septillion",
    "positive_one_over_octillion",
    "positive_one_over_nonillion",
    "positive_one_over_decillion",
    "negative_decillion",
    "negative_nonillion",
    "negative_octillion",
    "negative_septillion",
    "negative_sextillion",
    "negative_quintillion",
    "negative_quadrillion",
    "negative_trillion",
    "negative_billion",
    "negative_million",
    "negative_thousand",
    "negative_peanut",
    "negative_one_over_peanut",
    "negative_one_over_thousand",
    "negative_one_over_million",
    "negative_one_over_billion",
    "negative_one_over_trillion",
    "negative_one_over_quadrillion",
    "negative_one_over_quintillion",
    "negative_one_over_sextillion",
    "negative_one_over_septillion",
    "negative_one_over_octillion",
    "negative_one_over_nonillion",
    "negative_one_over_decillion",
];

/// Emits `compile_error!` tokens with an escaped message.
fn compile_error_tokens(message: &str) -> TokenStream {
    let escaped = message.replace('\\', "\\\\").replace('"', "\\\"");
    TokenStream::from_str(&format!("compile_error!(\"{escaped}\");")).expect("compile_error! construction should always parse")
}

/// Parses generated source into a token stream, returning compile-error tokens on failure.
fn parse_tokens(source: &str) -> TokenStream {
    match TokenStream::from_str(source) {
        Ok(tokens) => tokens,
        Err(err) => compile_error_tokens(&format!("base_mod_macros parse failure: {err}")),
    }
}

/// Parses an optional trait name argument from the macro input.
fn parse_requested_trait_name_or(input: TokenStream, default_name: &str) -> String {
    let requested_name = input.to_string();
    if requested_name.trim().is_empty() {
        default_name.to_string()
    } else {
        requested_name.trim().to_string()
    }
}

fn constants_root_from_manifest_dir() -> Result<PathBuf, String> {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").map_err(|err| format!("missing CARGO_MANIFEST_DIR while generating UsfScalar constant values trait: {err}"))?;
    Ok(PathBuf::from(manifest_dir).join("src/math/scalar/constants"))
}

fn extract_pub_const_names(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cursor = 0_usize;

    while let Some(pos) = source[cursor..].find("pub const ") {
        let const_kw_idx = cursor + pos;
        let mut ident_start = const_kw_idx + "pub const ".len();
        let bytes = source.as_bytes();
        while ident_start < source.len() && bytes[ident_start].is_ascii_whitespace() {
            ident_start += 1;
        }

        let mut ident_end = ident_start;
        while ident_end < source.len() {
            let ch = bytes[ident_end];
            if ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == b'_' {
                ident_end += 1;
            } else {
                break;
            }
        }

        if ident_end > ident_start {
            out.push(source[ident_start..ident_end].to_string());
        }
        cursor = ident_end.max(const_kw_idx + 1);
    }

    out
}

fn parse_pub_const_names_in_file(path: &Path) -> Result<Vec<String>, String> {
    let source = fs::read_to_string(path).map_err(|err| format!("failed reading constants source file `{}`: {err}", path.display()))?;
    let names = extract_pub_const_names(&source);
    if names.is_empty() {
        return Err(format!("failed to discover `pub const` scalar constants in `{}`", path.display()));
    }
    Ok(names)
}

#[derive(Debug)]
struct ConstantSpec {
    const_name: String,
    const_path: String,
}

fn all_constant_specs() -> Result<Vec<ConstantSpec>, String> {
    let constants_root = constants_root_from_manifest_dir()?;
    let mut specs = Vec::new();

    let core_path = constants_root.join("core.rs");
    for const_name in parse_pub_const_names_in_file(&core_path)? {
        specs.push(ConstantSpec {
            const_path: format!("crate::math::scalar::constants::core::{const_name}"),
            const_name,
        });
    }

    let range_root = constants_root.join("range_sample");
    for module in RANGE_SAMPLE_MODULES {
        let module_path = range_root.join(format!("{module}.rs"));
        for const_name in parse_pub_const_names_in_file(&module_path)? {
            specs.push(ConstantSpec {
                const_path: format!("crate::math::scalar::constants::range_sample::{module}::{const_name}"),
                const_name,
            });
        }
    }

    Ok(specs)
}

/// Declares and implements a giant `UsfScalar` constant-values trait generated from raw scalar constants files.
///
/// This emits:
/// - `pub trait {TraitName}` with one associated `UsfScalar` constant per raw scalar constant.
/// - `impl {TraitName} for crate::math::scalar::usf::UsfScalar` that converts each raw `ScalarCoreConst`
///   through `ScalarDecimalDigits::from_balanced_parts_const_checked(...)`.
pub(crate) fn declare_usf_scalar_constant_values_trait(input: TokenStream) -> TokenStream {
    let trait_name = parse_requested_trait_name_or(input, "UsfScalarConstants");
    let specs = match all_constant_specs() {
        Ok(specs) => specs,
        Err(err) => return compile_error_tokens(&format!("declare_usf_scalar_constant_values_trait! failed: {err}")),
    };

    let mut seen_constants = HashSet::<String>::new();
    let mut trait_constants = String::new();
    let mut impl_constants = String::new();

    for spec in specs {
        let constant_name = spec.const_name.to_ascii_uppercase();
        if !seen_constants.insert(constant_name.clone()) {
            return compile_error_tokens(&format!(
                "declare_usf_scalar_constant_values_trait! generated duplicate constant `{constant_name}`"
            ));
        }

        trait_constants.push_str(&format!("    const {constant_name}: crate::math::scalar::usf::UsfScalar;\n"));
        impl_constants.push_str(&format!(
            "    const {constant_name}: crate::math::scalar::usf::UsfScalar = {{\n        let (int_balanced, frac_balanced_internal, balanced_negative, _) = {};\n        Self {{\n            digits: crate::math::scalar::digits::ScalarDecimalDigits::from_balanced_parts_const_checked(\n                balanced_negative,\n                int_balanced,\n                frac_balanced_internal,\n            ),\n        }}\n    }};\n",
            spec.const_path
        ));
    }

    let out = format!("pub trait {trait_name} {{\n{trait_constants}}}\n\nimpl {trait_name} for crate::math::scalar::usf::UsfScalar {{\n{impl_constants}}}\n");

    parse_tokens(&out)
}
