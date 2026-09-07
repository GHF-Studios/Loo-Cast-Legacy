use std::fmt::{Display, Formatter};

/// Error returned when converting a decimal literal into word tokens fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalWordNameError {
    /// Input was empty after trimming.
    EmptyInput,
    /// Input did not match supported decimal/scientific syntax.
    InvalidNumericLiteral(String),
    /// Integer magnitude exceeded supported naming depth.
    UnsupportedIntegerMagnitude { digits: usize, max_supported_digits: usize },
    /// Fractional magnitude exceeded supported naming depth.
    UnsupportedFractionalMagnitude { digits: usize, max_supported_digits: usize },
    /// Value falls outside the scalar model's asymmetric representable range.
    OutOfRepresentableRange,
}

impl Display for DecimalWordNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "empty numeric literal"),
            Self::InvalidNumericLiteral(text) => write!(f, "invalid numeric literal: {text}"),
            Self::UnsupportedIntegerMagnitude { digits, max_supported_digits } => {
                write!(f, "integer magnitude exceeds supported depth: {digits} digits (max {max_supported_digits})")
            }
            Self::UnsupportedFractionalMagnitude { digits, max_supported_digits } => {
                write!(f, "fractional magnitude exceeds supported depth: {digits} digits (max {max_supported_digits})")
            }
            Self::OutOfRepresentableRange => write!(f, "numeric literal exceeds scalar model representable range"),
        }
    }
}

impl std::error::Error for DecimalWordNameError {}
