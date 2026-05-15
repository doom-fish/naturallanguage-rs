# naturallanguage

Safe Rust bindings for Apple's [NaturalLanguage](https://developer.apple.com/documentation/naturallanguage) framework on macOS — language detection, tokenisation, named-entity recognition.

> **Status:** experimental. v0.1 ships single-shot dominant-language detection, multi-hypothesis ranking, word/sentence/paragraph/document tokenisation, and PersonalName/PlaceName/OrganizationName entity extraction. Embeddings (`NLEmbedding`), gazetteer-driven matching (`NLGazetteer`), and custom-model loading (`NLModel`) land in v0.2.

## Quick start

```rust,no_run
use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Language detection
    let lang = dominant_language("Tim Cook visited Apple Park.")?;
    println!("dominant: {lang:?}");                         // Some("en")

    // 2. Multi-hypothesis (top 3)
    let hyps = language_hypotheses("Bonjour le monde", 3)?;
    for h in &hyps {
        println!("  {} ({:.2})", h.language, h.confidence); // fr (0.99) ...
    }

    // 3. Tokenisation
    let tokens = tokenize("The quick brown fox.", TokenUnit::Word)?;
    println!("words: {:?}",
        tokens.iter().map(|t| t.text.as_str()).collect::<Vec<_>>());

    // 4. Named-entity recognition
    let ents = named_entities(
        "Tim Cook visited Apple Park in Cupertino last Tuesday."
    )?;
    for e in &ents {
        println!("  {:?}: {}", e.kind, e.text);
        // PersonalName: Tim Cook
        // OrganizationName: Apple
        // PlaceName: Cupertino
    }
    Ok(())
}
```

## Pipeline composition

```text
screencapturekit-rs ──► system audio ──► speech ──► transcript
                                                       │
                                                       ▼
                            ┌──────── naturallanguage ────────┐
                            │                                  │
                            ▼              ▼                    ▼
                      detect language  tokenize          extract entities
                            │              │                    │
                            └──────────────┴────────────────────┘
                                           │
                                           ▼
                                  foundation-models
                                  ("summarise these utterances by speaker")
```

Pairs naturally with [`speech`](https://github.com/doom-fish/speech-rs) and [`foundation-models`](https://github.com/doom-fish/foundation-models-rs) for end-to-end on-device transcription → understanding pipelines.

## Feature flags

* `language_detection` (default) — `NLLanguageRecognizer`
* `tokenize` (default) — `NLTokenizer`
* `tag` (default) — `NLTagger` with the `.nameType` scheme

Disable any to shrink the surface; the Swift bridge always links the full framework.

## Roadmap

- [x] Dominant-language detection (`NLLanguageRecognizer.dominantLanguage`)
- [x] Ranked multi-hypothesis (`languageHypothesesWithMaximum:`)
- [x] Word / sentence / paragraph / document tokenisation
- [x] Named-entity recognition (`PersonalName` / `PlaceName` / `OrganizationName`)
- [ ] Lexical-class tagging (POS, lemmas) via other `NLTagScheme`s
- [ ] `NLEmbedding` — sentence + word vectors
- [ ] `NLGazetteer` — keyword-driven custom recogniser
- [ ] `NLModel` — load + run `.mlmodel` text classifiers
- [ ] Async API

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
