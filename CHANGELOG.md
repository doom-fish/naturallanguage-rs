# Changelog

All notable changes to `naturallanguage` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - Unreleased

### Security

- `Tagger`, `Tokenizer` and `LanguageRecognizer` are no longer `Sync`. Their headers
  forbid using one instance from two threads at once, so `&self` queries through an
  `Arc` could race inside the framework.
- Caller ranges can no longer abort the process. `token_range_for_range`,
  `set_language`, `set_orthography` and the contextual `token_vectors_in_range`
  skipped validation, so `TextRange::new(1, usize::MAX)` reached Swift as a negative
  length and trapped; the shared validation itself trapped on `location + length`
  overflow. Every range-taking call now validates with checked arithmetic and
  returns `NLError::InvalidArgument`.

### Fixed

- `Tagger::request_assets` and `ContextualEmbedding::request_embedding_assets` report
  a timeout as `NLError::TimedOut` instead of success with an empty `Error` result.
  They wait up to 300 s (was 30 s), the completion writes into lock-protected state,
  and the error message that accompanies an `Error` result is freed instead of
  leaked.
- A framework error code outside the `Int32` range no longer traps, and codes that
  fall into the bridge's status range are reported as `Unknown` rather than, for
  example, `InvalidArgument`.
- `CoreMlModel::from_source_path` deletes the compiled `.mlmodelc` it creates in
  `TMPDIR` once the last reference to the model is gone.
- `TextRange::end` saturates instead of overflowing.

### Changed

- **Breaking:** `Tagger`, `Tokenizer` and `LanguageRecognizer` are `Send` but not
  `Sync`.
- **Breaking:** `Tagger::tags_in_range` and `Tokenizer::tokens_in_range` return
  `NLError::InvalidArgument` for an out-of-bounds range or a missing string instead
  of an empty list.
- `rust-version` is 1.82 (was 1.76, which never compiled: `TagSpanRaw` derived
  `Default` over raw pointers, a Rust 1.88 feature; it now implements it by hand).
  `doom-fish-utils` is required at `>=0.4.1, <0.5`.
- FFI hardening that was not yet released: compile-time size and alignment checks
  for the FFI structs with a Swift-side cross-check (`nl_verify_ffi_layout`), one
  macro for the retain/release boilerplate, and no empty bridge header.

### Added

- `TextRange::byte_range` converts a UTF-16 range to a byte range of a Rust `&str`.
- `NLError::TimedOut` and `ffi::status::TIMED_OUT`.

## [0.4.4] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.4.3] - 2026-05-20

- Phase 32 completeness + async sweep.
- Added the optional `async` feature/module with executor-agnostic futures for `NLTagger.requestAssetsForLanguage(_:tagScheme:completionHandler:)` and `NLContextualEmbedding.requestEmbeddingAssetsWithCompletionHandler(_:)`.
- Added retained cloning support for `ContextualEmbedding` handles and refreshed the coverage docs against `MacOSX26.5.sdk`.

## [0.4.2] - 2026-05-17

### Changed

- Added documentation comments to `unsafe impl Send/Sync` blocks explaining thread-safety guarantees for all wrapper types (`Embedding`, `LanguageRecognizer`, `Tokenizer`, `Tagger`, `Gazetteer`, `ContextualEmbedding`, `ContextualEmbeddingResult`, `CoreMlModel`, `ModelConfiguration`, `Model`). These types wrap Objective-C framework objects, which are thread-safe by design.

## [0.4.1] - 2026-05-17

### Added

- Seven end-to-end integration tests under `tests/`, covering `NLLanguageRecognizer`, `NLTokenizer`, `NLTagger`, `NLEmbedding`, `NLContextualEmbedding`, `NLModel`, and `NLGazetteer`.

### Changed

- Bumped the crate and release metadata to `v0.4.1`.

## [0.4.0] - 2026-05-16

### Added

- `COVERAGE.md`, a row-by-row audit of the public `NaturalLanguage.framework` macOS 26.2 SDK surface against the crate's expanded public API.
- `embedding::Distance` / `naturallanguage::Distance` as the Rust alias for Apple's public `NLDistance` typedef.
- An explicit SDK audit assertion that `NLDataAsset` is absent from the current macOS `NaturalLanguage.framework` headers, so future SDK additions fail loudly.

### Changed

- Refreshed the README and release metadata for the audited `v0.4.0` release.

## [0.3.0] - 2026-05-16

### Added

- Full audited wrapper coverage for the public `NaturalLanguage.framework` macOS SDK surface.
- Typed `Language`, `Script`, `Tag`, `TagScheme`, and `TextRange` support.
- Stateful `LanguageRecognizer`, `Tokenizer`, and expanded `Tagger` APIs while preserving the original convenience helpers.
- `NLGazetteer`, `NLModel` / `NLModelConfiguration`, and `NLContextualEmbedding` wrappers.
- Expanded `NLEmbedding` support for revisions, file-backed loading, neighbor/vector queries, and dictionary export.
- Header-based API coverage tests spanning constants, enums, and object APIs.
- New smoke examples for advanced tagging, gazetteers, custom models, and contextual embeddings.
- Bundled CreateML-generated `.mlmodel` fixtures under `examples/assets/` for model smoke tests.

### Fixed

- `named_entities` no longer double-frees tag spans when decoding `NLTagger` results.

## [0.1.0] - Initial release

### Added

- `dominant_language(text)` — wraps `NLLanguageRecognizer.dominantLanguage`.
- `language_hypotheses(text, max)` — ranked (lang, confidence) pairs from
  `NLLanguageRecognizer.languageHypothesesWithMaximum:`.
- `tokenize(text, TokenUnit)` — wraps `NLTokenizer` for word / sentence /
  paragraph / document units. Returns `Vec<Token { start, length, text }> `.
- `named_entities(text)` — wraps `NLTagger` with the `.nameType` scheme +
  `[.omitPunctuation, .omitWhitespace, .joinNames]` options. Returns
  `Vec<NamedEntity { start, length, text, kind }>` filtered to
  `EntityKind::{PersonalName, PlaceName, OrganizationName}`.
- `NLError` — `InvalidArgument` + `Unknown { code, message }` catch-all.
- 3 examples: `01_detect_language`, `02_tokenize`, `03_named_entities`.
- 3 API-coverage tests (`NLLanguageRecognizer`, `NLTokenizer`, `NLTagger`)
  using the family's Obj-C `@interface` header-parsing pattern.
- Feature flags `language_detection` + `tokenize` + `tag` (all default on).
