//! `base_mod_macros`
//!
//! Procedural macros used by `base_mod_*` crates.
//!
//! Constants macro
//! - `declare_usf_scalar_constant_values_trait!([TraitName])`: declares a macro-generated
//!   trait containing one `UsfScalar` associated constant per raw scalar constant.

extern crate proc_macro;

mod constants;

use proc_macro::TokenStream;

/// Declares + implements a trait that exposes every scalar constant as a `UsfScalar` associated constant.
///
/// # Input
/// - Empty: `declare_usf_scalar_constant_values_trait!()` generates `pub trait UsfScalarKnownConstants`.
/// - Trait name: `declare_usf_scalar_constant_values_trait!(MyTrait)` generates `pub trait MyTrait`.
#[proc_macro]
pub fn declare_usf_scalar_constant_values_trait(input: TokenStream) -> TokenStream {
    constants::declare_usf_scalar_constant_values_trait(input)
}
