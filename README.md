# naturallanguage

Safe Rust bindings for Apple's [NaturalLanguage](https://developer.apple.com/documentation/naturallanguage) framework on macOS — language detection, tokenization, tagging, embeddings, gazetteers, and custom/Core ML-backed language models.

> **Status:** experimental. `v0.3.0` covers the full public `NaturalLanguage.framework` header surface in the current macOS SDK audit, including `NLLanguage`, `NLScript`, `NLLanguageRecognizer`, `NLTokenizer`, `NLTagger`, `NLEmbedding`, `NLGazetteer`, `NLModel`, and `NLContextualEmbedding`. Availability-gated APIs return `NLError::Unsupported` on older macOS releases.

## Quick start

```rust,no_run
use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Tim Cook visited Apple Park in Cupertino.";

    let mut recognizer = LanguageRecognizer::new()?;
    recognizer.process(text)?;
    println!("dominant language: {:?}", recognizer.dominant_language()?);

    let mut tokenizer = Tokenizer::new(TokenUnit::Word)?;
    tokenizer.set_string(Some(text))?;
    for token in tokenizer.tokens_in_range(TextRange::new(0, text.encode_utf16().count()))? {
        println!("token: {:?}", token.text);
    }

    for entity in named_entities(text)? {
        println!("entity: {:?} -> {}", entity.kind, entity.text);
    }

    Ok(())
}
```

## Included surface

- Typed extensible enums for `Language`, `Script`, `Tag`, and `TagScheme`
- Stateful and convenience wrappers for `NLLanguageRecognizer`
- Stateful and convenience wrappers for `NLTokenizer`
- Full `NLTagger` object API, schemes, options, orthography, model attachment, and gazetteer attachment
- `NLEmbedding` word/sentence embeddings, revisions, file-backed loading, and dictionary export
- `NLGazetteer` creation/loading/serialization APIs
- `NLModel` and `NLModelConfiguration` wrappers plus minimal `MLModel` interop
- `NLContextualEmbedding` and `NLContextualEmbeddingResult` (macOS 14+)

## Feature flags

All features are enabled by default.

- `language_detection` — `NLLanguageRecognizer`
- `tokenize` — `NLTokenizer`
- `tag` — `NLTagger`
- `embedding` — `NLEmbedding`
- `gazetteer` — `NLGazetteer`
- `model` — `NLModel` / `MLModel`
- `contextual_embedding` — `NLContextualEmbedding`

## Smoke examples

- `cargo run --example 01_detect_language`
- `cargo run --example 02_tokenize`
- `cargo run --example 03_named_entities`
- `cargo run --example 04_embedding`
- `cargo run --example 05_tagger`
- `cargo run --example 06_gazetteer`
- `cargo run --example 07_model`
- `cargo run --example 08_contextual_embedding`

`07_model` uses bundled CreateML-generated fixtures under `examples/assets/` and compiles them with `xcrun coremlcompiler` before loading them through `NLModel`.

## Verification

The crate is validated with:

- `cargo build --all-features`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- `swift build --package-path swift-bridge -c release`

## Roadmap

- [x] Full public header coverage for the current `NaturalLanguage.framework` SDK
- [x] Header-based API coverage tests for constants, enums, and object APIs
- [x] End-to-end smoke examples for each major feature area
- [ ] Higher-level async/convenience layers on top of the low-level bindings

Pairs naturally with [`speech`](https://github.com/doom-fish/speech-rs) and [`foundation-models`](https://github.com/doom-fish/foundation-models-rs) for on-device transcription → understanding pipelines.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
