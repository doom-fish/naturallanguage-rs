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
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod string_enum;
pub(crate) mod util;

pub mod error;
pub mod ffi;
pub mod language;
pub mod script;
pub mod types;

#[cfg(feature = "language_detection")]
#[cfg_attr(docsrs, doc(cfg(feature = "language_detection")))]
pub mod recognizer;

#[cfg(feature = "tokenize")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokenize")))]
pub mod tokenizer;

#[cfg(feature = "tag")]
#[cfg_attr(docsrs, doc(cfg(feature = "tag")))]
pub mod tagger;

#[cfg(feature = "embedding")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedding")))]
pub mod embedding;

#[cfg(feature = "gazetteer")]
#[cfg_attr(docsrs, doc(cfg(feature = "gazetteer")))]
pub mod gazetteer;

#[cfg(feature = "model")]
#[cfg_attr(docsrs, doc(cfg(feature = "model")))]
pub mod model;

#[cfg(feature = "contextual_embedding")]
#[cfg_attr(docsrs, doc(cfg(feature = "contextual_embedding")))]
pub mod contextual_embedding;

pub use error::NLError;
pub use language::Language;
pub use script::Script;
pub use types::TextRange;

#[cfg(feature = "language_detection")]
pub use recognizer::{dominant_language, language_hypotheses, LanguageHypothesis, LanguageRecognizer};

#[cfg(feature = "tokenize")]
pub use tokenizer::{tokenize, Token, TokenSpan, TokenUnit, Tokenizer, TokenizerAttributes};

#[cfg(feature = "tag")]
pub use tagger::{
    named_entities, EntityKind, NamedEntity, Orthography, Tag, TagHypothesis, TagScheme, TaggedRange,
    Tagger, TaggerAssetsResult, TaggerOptions,
};

#[cfg(feature = "embedding")]
pub use embedding::{Embedding, EmbeddingDictionaryEntry, DistanceType, Neighbor};

#[cfg(feature = "gazetteer")]
pub use gazetteer::Gazetteer;

#[cfg(feature = "model")]
pub use model::{CoreMlModel, Model, ModelConfiguration, ModelType};

#[cfg(feature = "contextual_embedding")]
pub use contextual_embedding::{
    ContextualEmbedding, ContextualEmbeddingAssetsResult, ContextualEmbeddingQuery,
    ContextualEmbeddingResult, TokenVector,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::NLError;
    pub use crate::language::Language;
    pub use crate::script::Script;
    pub use crate::types::TextRange;
    #[cfg(feature = "language_detection")]
    pub use crate::recognizer::{dominant_language, language_hypotheses, LanguageHypothesis, LanguageRecognizer};
    #[cfg(feature = "tokenize")]
    pub use crate::tokenizer::{tokenize, Token, TokenSpan, TokenUnit, Tokenizer, TokenizerAttributes};
    #[cfg(feature = "tag")]
    pub use crate::tagger::{
        named_entities, EntityKind, NamedEntity, Orthography, Tag, TagHypothesis, TagScheme,
        TaggedRange, Tagger, TaggerAssetsResult, TaggerOptions,
    };
    #[cfg(feature = "embedding")]
    pub use crate::embedding::{Embedding, EmbeddingDictionaryEntry, DistanceType, Neighbor};
    #[cfg(feature = "gazetteer")]
    pub use crate::gazetteer::Gazetteer;
    #[cfg(feature = "model")]
    pub use crate::model::{CoreMlModel, Model, ModelConfiguration, ModelType};
    #[cfg(feature = "contextual_embedding")]
    pub use crate::contextual_embedding::{
        ContextualEmbedding, ContextualEmbeddingAssetsResult, ContextualEmbeddingQuery,
        ContextualEmbeddingResult, TokenVector,
    };
}
