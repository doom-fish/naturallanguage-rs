# Changelog

## [0.1.0] - Initial release

### Added

- `dominant_language(text)` — wraps `NLLanguageRecognizer.dominantLanguage`.
- `language_hypotheses(text, max)` — ranked (lang, confidence) pairs from
  `NLLanguageRecognizer.languageHypothesesWithMaximum:`.
- `tokenize(text, TokenUnit)` — wraps `NLTokenizer` for word / sentence /
  paragraph / document units. Returns `Vec<Token { start, length, text }>`.
- `named_entities(text)` — wraps `NLTagger` with the `.nameType` scheme +
  `[.omitPunctuation, .omitWhitespace, .joinNames]` options. Returns
  `Vec<NamedEntity { start, length, text, kind }>` filtered to
  `EntityKind::{PersonalName, PlaceName, OrganizationName}`.
- `NLError` — `InvalidArgument` + `Unknown { code, message }` catch-all.
- 3 examples: `01_detect_language`, `02_tokenize`, `03_named_entities`.
- 3 API-coverage tests (`NLLanguageRecognizer`, `NLTokenizer`, `NLTagger`)
  using the family's Obj-C `@interface` header-parsing pattern.
- Feature flags `language_detection` + `tokenize` + `tag` (all default on).
