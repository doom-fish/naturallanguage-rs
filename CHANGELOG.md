# Changelog

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
