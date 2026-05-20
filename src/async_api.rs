//! Executor-agnostic async helpers for `naturallanguage`.
//!
//! Enable with `features = ["async"]`.
//!
//! | Rust surface | Apple API |
//! |---|---|
//! | [`Tagger::request_assets_async`] | `NLTagger.requestAssets(for:tagScheme:completionHandler:)` |
//! | [`ContextualEmbedding::request_embedding_assets_async`] | `NLContextualEmbedding.requestAssets(completionHandler:)` |

#[cfg(feature = "contextual_embedding")]
pub use crate::contextual_embedding::{
    ContextualEmbedding, ContextualEmbeddingAssetsFuture, ContextualEmbeddingAssetsResult,
};
#[cfg(feature = "tag")]
pub use crate::tagger::{Tagger, TaggerAssetsFuture, TaggerAssetsResult, TagScheme};
pub use crate::{Language, NLError};
