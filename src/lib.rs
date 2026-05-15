#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [NaturalLanguage](https://developer.apple.com/documentation/naturallanguage)
//! framework on macOS — language detection, tokenisation, named-entity
//! recognition.
//!
//! All processing is on-device with no model downloads.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod ffi;

#[cfg(feature = "language_detection")]
#[cfg_attr(docsrs, doc(cfg(feature = "language_detection")))]
pub mod recognizer;

#[cfg(feature = "tokenize")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokenize")))]
pub mod tokenizer;

#[cfg(feature = "tag")]
#[cfg_attr(docsrs, doc(cfg(feature = "tag")))]
pub mod tagger;

pub use error::NLError;

#[cfg(feature = "language_detection")]
pub use recognizer::{dominant_language, language_hypotheses, LanguageHypothesis};

#[cfg(feature = "tokenize")]
pub use tokenizer::{tokenize, Token, TokenUnit};

#[cfg(feature = "tag")]
pub use tagger::{named_entities, EntityKind, NamedEntity};

/// Common imports.
pub mod prelude {
    pub use crate::error::NLError;
    #[cfg(feature = "language_detection")]
    pub use crate::recognizer::{dominant_language, language_hypotheses, LanguageHypothesis};
    #[cfg(feature = "tokenize")]
    pub use crate::tokenizer::{tokenize, Token, TokenUnit};
    #[cfg(feature = "tag")]
    pub use crate::tagger::{named_entities, EntityKind, NamedEntity};
}
