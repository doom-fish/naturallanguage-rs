//! Errors returned by the `naturallanguage` bridge.

use core::fmt;

/// Errors returned by the public API.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NLError {
    /// Caller supplied an invalid argument (e.g. NUL byte in input).
    InvalidArgument(String),
    /// Catch-all for unmapped statuses from the Swift bridge.
    Unknown { code: i32, message: String },
}

impl fmt::Display for NLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
            Self::Unknown { code, message } => write!(f, "naturallanguage error {code}: {message}"),
        }
    }
}

impl std::error::Error for NLError {}
