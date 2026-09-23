# naturallanguage

Safe Rust bindings for Apple's [NaturalLanguage](https://developer.apple.com/documentation/naturallanguage) framework on macOS — language detection, tokenization, tagging, embeddings, gazetteers, and custom/Core ML-backed language models.

> **Status:** experimental. The full public `NaturalLanguage.framework` header surface in `MacOSX26.5.sdk` is wrapped, with row-by-row results in [`COVERAGE.md`](COVERAGE.md). `NLDataAsset` is not present in the current macOS headers, so it is recorded there as skipped/absent. Requires macOS 13 or later; `NLContextualEmbedding` needs macOS 14, and availability-gated APIs return `NLError::Unsupported` on older releases.

## Installation

```toml
[dependencies]
naturallanguage = "0.5"
```

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
- `NLEmbedding` word/sentence embeddings, `Distance` / `DistanceType`, revisions, file-backed loading, and dictionary export
- Row-by-row SDK audit coverage in [`COVERAGE.md`](COVERAGE.md)
- `NLGazetteer` creation/loading/serialization APIs
- `NLModel` and `NLModelConfiguration` wrappers plus minimal `MLModel` interop
- `NLContextualEmbedding` and `NLContextualEmbeddingResult` (macOS 14+)
- Optional executor-agnostic futures for `NLTagger` / `NLContextualEmbedding` asset requests

## Ranges, threads and asset requests

- Ranges and indices are UTF-16 offsets, as in Apple's `NSRange` APIs. `TextRange::byte_range(text)` converts a range to a byte range for slicing a Rust `&str`, and returns `None` if it would split a character.
- A range or index outside the current string is rejected with `NLError::InvalidArgument`, including lengths such as `usize::MAX`.
- `Tagger`, `Tokenizer`, and `LanguageRecognizer` are `Send` but not `Sync`: Apple allows each instance on one thread at a time. The embedding, gazetteer, and model types are `Send + Sync`.
- `Tagger::request_assets` and `ContextualEmbedding::request_embedding_assets` block until the framework answers, for up to 300 seconds, and then return `NLError::TimedOut`. The download continues in the background.

## Feature flags

All features are enabled by default.

- `language_detection` — `NLLanguageRecognizer`
- `tokenize` — `NLTokenizer`
- `tag` — `NLTagger`
- `embedding` — `NLEmbedding`
- `gazetteer` — `NLGazetteer`
- `model` — `NLModel` / `MLModel`
- `contextual_embedding` — `NLContextualEmbedding`
- `async` — executor-agnostic futures for one-shot asset requests

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

- `cargo expand --lib > target/cargo-expand-lib.rs`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`
- `swift build --package-path swift-bridge -c release`

## Roadmap

- [x] Full public header coverage for the current `NaturalLanguage.framework` SDK, with the audit published in [`COVERAGE.md`](COVERAGE.md)
- [x] Header-based API coverage tests for constants, enums, and object APIs
- [x] End-to-end smoke examples for each major feature area
- [x] Executor-agnostic async futures for one-shot asset requests
- [ ] Broader async/convenience layers on top of the low-level bindings

Pairs naturally with [`speech`](https://github.com/doom-fish/speech-rs) and [`foundation-models`](https://github.com/doom-fish/foundation-models-rs) for on-device transcription → understanding pipelines.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
