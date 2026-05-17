//! API-surface coverage harness for `naturallanguage`.
//!
//! This checks the crate's public Rust + Swift bridge surface against the
//! public `NaturalLanguage.framework` headers shipped in the active macOS SDK.

#![allow(clippy::cast_precision_loss)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn read_files(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| read(path))
        .collect::<Vec<_>>()
        .join("\n")
}

fn rust_sources(files: &[&str]) -> String {
    let base = repo_root().join("src");
    let paths = files.iter().map(|file| base.join(file)).collect::<Vec<_>>();
    read_files(&paths)
}

fn bridge_sources(files: &[&str]) -> String {
    let base = repo_root().join("swift-bridge/Sources/NaturalLanguageBridge");
    let paths = files.iter().map(|file| base.join(file)).collect::<Vec<_>>();
    read_files(&paths)
}

fn read_header(name: &str) -> String {
    read(&sdk_root().join(format!(
        "System/Library/Frameworks/NaturalLanguage.framework/Headers/{name}.h"
    )))
}

fn extract_interface(header: &str, type_name: &str) -> String {
    let needle = regex_lite::Regex::new(&format!(r"@interface\s+{type_name}\b")).unwrap();
    let Some(start) = needle.find(header) else {
        return String::new();
    };
    let rest = &header[start.start()..];
    let Some(end_off) = rest.find("@end") else {
        return rest.to_string();
    };
    rest[..end_off].to_string()
}

fn method_selector(signature: &str) -> String {
    let label_re = regex_lite::Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)\s*:").unwrap();
    let labels = label_re
        .captures_iter(signature)
        .map(|capture| capture[1].to_string())
        .collect::<Vec<_>>();
    if labels.is_empty() {
        signature
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .trim()
            .to_string()
    } else {
        let mut selector = String::new();
        for label in labels {
            selector.push_str(&label);
            selector.push(':');
        }
        selector
    }
}

fn extract_member_surface(interface_body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();

    let method_re = regex_lite::Regex::new(r"(?m)^\s*[+\-]\s*\([^\)]*\)\s*([^\n;]+)").unwrap();
    for capture in method_re.captures_iter(interface_body) {
        let signature = capture[1]
            .split(" API_")
            .next()
            .unwrap()
            .split(" NS_")
            .next()
            .unwrap()
            .trim();
        out.insert(method_selector(signature));
    }

    let prop_re = regex_lite::Regex::new(
        r"(?m)^\s*@property\s*(?:\([^\)]*\))?\s*[^;]*?\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:API_|NS_|;)",
    )
    .unwrap();
    for capture in prop_re.captures_iter(interface_body) {
        out.insert(capture[1].to_string());
    }

    let getter_re = regex_lite::Regex::new(r"getter\s*=\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for capture in getter_re.captures_iter(interface_body) {
        out.insert(capture[1].to_string());
    }

    out
}

fn extract_exported_constants(header: &str, type_name: &str) -> BTreeSet<String> {
    let regex = regex_lite::Regex::new(&format!(
        r"FOUNDATION_EXPORT\s+{}\s+const\s+([A-Za-z_][A-Za-z0-9_]*)",
        regex_lite::escape(type_name)
    ))
    .unwrap();
    regex
        .captures_iter(header)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn extract_c_enum_cases(header: &str, enum_name: &str, case_prefix: &str) -> BTreeSet<String> {
    let start_re = regex_lite::Regex::new(&format!(
        r"typedef\s+NS_(?:ENUM|OPTIONS)\s*\([^\)]*,\s*{}\)\s*\{{",
        regex_lite::escape(enum_name)
    ))
    .unwrap();
    let Some(start) = start_re.find(header) else {
        return BTreeSet::new();
    };
    let rest = &header[start.end()..];
    let Some(end) = rest.find("};") else {
        return BTreeSet::new();
    };
    let body = &rest[..end];
    let case_re = regex_lite::Regex::new(&format!(
        r"\b({}[A-Za-z0-9_]+)\b",
        regex_lite::escape(case_prefix)
    ))
    .unwrap();
    case_re
        .captures_iter(body)
        .map(|capture| capture[1].strip_prefix(case_prefix).unwrap().to_string())
        .collect()
}

fn to_upper_snake(name: &str) -> String {
    let mut out = String::new();
    let mut prev_is_lower_or_digit = false;
    for ch in name.chars() {
        if ch.is_uppercase() && prev_is_lower_or_digit {
            out.push('_');
        }
        prev_is_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        out.extend(ch.to_uppercase());
    }
    out
}

fn lower_first(name: &str) -> String {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_lowercase().collect::<String>() + chars.as_str()
}

fn rust_string_enum_constants(corpus: &str) -> BTreeSet<String> {
    let regex = regex_lite::Regex::new(r"(?m)^\s*([A-Z][A-Z0-9_]+)\s*=").unwrap();
    regex
        .captures_iter(corpus)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn rust_const_names(corpus: &str) -> BTreeSet<String> {
    let regex = regex_lite::Regex::new(r"(?m)\bconst\s+([A-Z][A-Z0-9_]+)\b").unwrap();
    regex
        .captures_iter(corpus)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn rust_enum_variants(corpus: &str) -> BTreeSet<String> {
    let regex = regex_lite::Regex::new(r"(?m)^\s*([A-Z][A-Za-z0-9]+)\s*(?:=|,)").unwrap();
    regex
        .captures_iter(corpus)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn alias_map(entries: &[(&str, &[&str])]) -> BTreeMap<String, Vec<String>> {
    entries
        .iter()
        .map(|(symbol, aliases)| {
            (
                (*symbol).to_string(),
                aliases.iter().map(|alias| (*alias).to_string()).collect(),
            )
        })
        .collect()
}

fn references_in_corpus(
    symbols: &BTreeSet<String>,
    corpus: &str,
    aliases: &BTreeMap<String, Vec<String>>,
) -> BTreeSet<String> {
    symbols
        .iter()
        .filter(|name| {
            aliases
                .get(name.as_str())
                .is_some_and(|patterns| patterns.iter().any(|pattern| corpus.contains(pattern)))
                || (!name.contains(':')
                    && regex_lite::Regex::new(&format!(r"\b{}\b", regex_lite::escape(name)))
                        .unwrap()
                        .is_match(corpus))
        })
        .cloned()
        .collect()
}

fn report(
    name: &str,
    apple: &BTreeSet<String>,
    ours: &BTreeSet<String>,
    omitted: &BTreeSet<String>,
) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple
        .difference(ours)
        .filter(|symbol| !omitted.contains(*symbol))
        .collect();
    let coverable = wrapped.len() + missing.len();
    let pct = if coverable == 0 {
        100.0
    } else {
        wrapped.len() as f64 / coverable as f64 * 100.0
    };
    println!(
        "\n=== {name} ===\n  apple={}, omitted={}, coverable={coverable}, wrapped={}, missing={}, pct={pct:.1}%",
        apple.len(),
        omitted.len(),
        wrapped.len(),
        missing.len(),
    );
    if !missing.is_empty() {
        for symbol in &missing {
            println!("  - {symbol}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

fn interface_coverage(
    name: &str,
    header_name: &str,
    type_name: &str,
    corpus: &str,
    aliases: &[(&str, &[&str])],
    omitted: &BTreeSet<String>,
) {
    let header = read_header(header_name);
    let apple = extract_member_surface(&extract_interface(&header, type_name));
    let ours = references_in_corpus(&apple, corpus, &alias_map(aliases));
    report(name, &apple, &ours, omitted);
}

#[test]
fn nllanguage_constants_coverage() {
    let header = read_header("NLLanguage");
    let apple = extract_exported_constants(&header, "NLLanguage")
        .into_iter()
        .map(|name| to_upper_snake(name.strip_prefix("NLLanguage").unwrap()))
        .collect::<BTreeSet<_>>();
    let ours = rust_string_enum_constants(&rust_sources(&["language.rs"]));
    report("NLLanguage constants", &apple, &ours, &BTreeSet::new());
}

#[test]
fn nlscript_constants_coverage() {
    let header = read_header("NLScript");
    let apple = extract_exported_constants(&header, "NLScript")
        .into_iter()
        .map(|name| to_upper_snake(name.strip_prefix("NLScript").unwrap()))
        .collect::<BTreeSet<_>>();
    let ours = rust_string_enum_constants(&rust_sources(&["script.rs"]));
    report("NLScript constants", &apple, &ours, &BTreeSet::new());
}

#[test]
fn nltag_constants_coverage() {
    let header = read_header("NLTagScheme");
    let corpus = rust_sources(&["tagger/mod.rs"]);
    let ours = rust_string_enum_constants(&corpus);

    let schemes = extract_exported_constants(&header, "NLTagScheme")
        .into_iter()
        .map(|name| to_upper_snake(name.strip_prefix("NLTagScheme").unwrap()))
        .collect::<BTreeSet<_>>();
    report("NLTagScheme constants", &schemes, &ours, &BTreeSet::new());

    let tags = extract_exported_constants(&header, "NLTag")
        .into_iter()
        .map(|name| to_upper_snake(name.strip_prefix("NLTag").unwrap()))
        .collect::<BTreeSet<_>>();
    report("NLTag constants", &tags, &ours, &BTreeSet::new());
}

#[test]
fn tokenizer_enum_coverage() {
    let header = read_header("NLTokenizer");
    let corpus = rust_sources(&["tokenizer/mod.rs"]);
    let variants = rust_enum_variants(&corpus);
    let consts = rust_const_names(&corpus);

    let token_units = extract_c_enum_cases(&header, "NLTokenUnit", "NLTokenUnit");
    report("NLTokenUnit", &token_units, &variants, &BTreeSet::new());

    let attributes = extract_c_enum_cases(&header, "NLTokenizerAttributes", "NLTokenizerAttribute")
        .into_iter()
        .map(|case| to_upper_snake(&case))
        .collect::<BTreeSet<_>>();
    report(
        "NLTokenizerAttributes",
        &attributes,
        &consts,
        &BTreeSet::new(),
    );
}

#[test]
fn tagger_enum_coverage() {
    let header = read_header("NLTagger");
    let corpus = rust_sources(&["tagger/mod.rs"]);
    let variants = rust_enum_variants(&corpus);
    let consts = rust_const_names(&corpus);

    let options = extract_c_enum_cases(&header, "NLTaggerOptions", "NLTagger")
        .into_iter()
        .map(|case| to_upper_snake(&case))
        .collect::<BTreeSet<_>>();
    report("NLTaggerOptions", &options, &consts, &BTreeSet::new());

    let assets = extract_c_enum_cases(&header, "NLTaggerAssetsResult", "NLTaggerAssetsResult");
    report("NLTaggerAssetsResult", &assets, &variants, &BTreeSet::new());
}

#[test]
fn embedding_enum_coverage() {
    let header = read_header("NLEmbedding");
    let variants = rust_enum_variants(&rust_sources(&["embedding/mod.rs"]));
    let distance = extract_c_enum_cases(&header, "NLDistanceType", "NLDistanceType");
    report("NLDistanceType", &distance, &variants, &BTreeSet::new());
}

#[test]
fn embedding_typedef_coverage() {
    let header = read_header("NLEmbedding");
    assert!(header.contains("typedef double NLDistance;"));
    let corpus = rust_sources(&["embedding/mod.rs"]);
    assert!(corpus.contains("pub type Distance = f64;"));
}

#[test]
fn model_enum_coverage() {
    let header = read_header("NLModel");
    let variants = rust_enum_variants(&rust_sources(&["model.rs"]));
    let model_types = extract_c_enum_cases(&header, "NLModelType", "NLModelType");
    report("NLModelType", &model_types, &variants, &BTreeSet::new());
}

#[test]
fn contextual_embedding_enum_coverage() {
    let header = read_header("NLContextualEmbedding");
    let corpus = rust_sources(&["contextual_embedding.rs"]);
    let variants = rust_enum_variants(&corpus);

    let assets = extract_c_enum_cases(
        &header,
        "NLContextualEmbeddingAssetsResult",
        "NLContextualEmbeddingAssetsResult",
    );
    report(
        "NLContextualEmbeddingAssetsResult",
        &assets,
        &variants,
        &BTreeSet::new(),
    );

    let keys = extract_exported_constants(&header, "NLContextualEmbeddingKey")
        .into_iter()
        .map(|name| lower_first(name.strip_prefix("NLContextualEmbeddingKey").unwrap()))
        .collect::<BTreeSet<_>>();
    let ours = keys
        .iter()
        .filter(|key| {
            regex_lite::Regex::new(&format!(r"\b{}\b", regex_lite::escape(key)))
                .unwrap()
                .is_match(&corpus)
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    report(
        "NLContextualEmbeddingKey constants",
        &keys,
        &ours,
        &BTreeSet::new(),
    );
}

#[test]
fn nl_language_recognizer_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["recognizer/mod.rs"]),
        bridge_sources(&["NaturalLanguage.swift", "RecognizerBridge.swift"]),
    );
    interface_coverage(
        "NLLanguageRecognizer",
        "NLLanguageRecognizer",
        "NLLanguageRecognizer",
        &corpus,
        &[
            ("dominantLanguageForString:", &["nl_dominant_language"]),
            ("init", &["nl_language_recognizer_create"]),
            ("processString:", &["nl_language_recognizer_process_string"]),
            ("reset", &["nl_language_recognizer_reset"]),
            (
                "dominantLanguage",
                &["nl_language_recognizer_dominant_language"],
            ),
            (
                "languageHypothesesWithMaximum:",
                &[
                    "nl_language_recognizer_language_hypotheses",
                    "nl_language_hypotheses",
                ],
            ),
            ("languageHints", &["nl_language_recognizer_language_hints"]),
            (
                "languageConstraints",
                &["nl_language_recognizer_language_constraints"],
            ),
        ],
        &BTreeSet::new(),
    );
}

#[test]
fn nl_tokenizer_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["tokenizer/mod.rs"]),
        bridge_sources(&["NaturalLanguage.swift", "TokenizerBridge.swift"]),
    );
    interface_coverage(
        "NLTokenizer",
        "NLTokenizer",
        "NLTokenizer",
        &corpus,
        &[
            ("initWithUnit:", &["nl_tokenizer_create"]),
            ("unit", &["nl_tokenizer_unit"]),
            (
                "string",
                &["nl_tokenizer_string", "nl_tokenizer_set_string"],
            ),
            ("setLanguage:", &["nl_tokenizer_set_language"]),
            ("tokenRangeAtIndex:", &["nl_tokenizer_token_range_at_index"]),
            (
                "tokenRangeForRange:",
                &["nl_tokenizer_token_range_for_range"],
            ),
            (
                "tokensForRange:",
                &["nl_tokenizer_tokens_in_range", "tokens_in_range"],
            ),
            (
                "enumerateTokensInRange:usingBlock:",
                &["nl_tokenizer_tokens_in_range", "enumerate_tokens_in_range"],
            ),
        ],
        &BTreeSet::new(),
    );
}

#[test]
fn nl_tagger_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["tagger/mod.rs"]),
        bridge_sources(&["NaturalLanguage.swift", "TaggerBridge.swift"]),
    );
    interface_coverage(
        "NLTagger",
        "NLTagger",
        "NLTagger",
        &corpus,
        &[
            ("initWithTagSchemes:", &["nl_tagger_create"]),
            ("tagSchemes", &["nl_tagger_tag_schemes"]),
            ("string", &["nl_tagger_string", "nl_tagger_set_string"]),
            (
                "availableTagSchemesForUnit:language:",
                &["nl_tagger_available_tag_schemes"],
            ),
            (
                "tokenRangeAtIndex:unit:",
                &["nl_tagger_token_range_at_index"],
            ),
            (
                "tokenRangeForRange:unit:",
                &["nl_tagger_token_range_for_range"],
            ),
            ("dominantLanguage", &["nl_tagger_dominant_language"]),
            (
                "enumerateTagsInRange:unit:scheme:options:usingBlock:",
                &["nl_tagger_tags_in_range", "enumerate_tags_in_range"],
            ),
            (
                "tagAtIndex:unit:scheme:tokenRange:",
                &["nl_tagger_tag_at_index"],
            ),
            (
                "tagsInRange:unit:scheme:options:tokenRanges:",
                &["nl_tagger_tags_in_range"],
            ),
            (
                "tagHypothesesAtIndex:unit:scheme:maximumCount:tokenRange:",
                &["nl_tagger_tag_hypotheses_at_index"],
            ),
            ("setLanguage:range:", &["nl_tagger_set_language"]),
            ("setOrthography:range:", &["nl_tagger_set_orthography"]),
            ("setModels:forTagScheme:", &["nl_tagger_set_models"]),
            ("modelsForTagScheme:", &["nl_tagger_models_for_tag_scheme"]),
            ("setGazetteers:forTagScheme:", &["nl_tagger_set_gazetteers"]),
            (
                "gazetteersForTagScheme:",
                &["nl_tagger_gazetteers_for_tag_scheme"],
            ),
            (
                "requestAssetsForLanguage:tagScheme:completionHandler:",
                &["nl_tagger_request_assets"],
            ),
        ],
        &BTreeSet::new(),
    );
}

#[test]
fn nl_embedding_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["embedding/mod.rs"]),
        bridge_sources(&["NaturalLanguage.swift", "EmbeddingExtras.swift"]),
    );
    interface_coverage(
        "NLEmbedding",
        "NLEmbedding",
        "NLEmbedding",
        &corpus,
        &[
            (
                "wordEmbeddingForLanguage:",
                &["nl_word_embedding_for_language"],
            ),
            (
                "wordEmbeddingForLanguage:revision:",
                &["nl_word_embedding_for_language_revision"],
            ),
            (
                "sentenceEmbeddingForLanguage:",
                &["nl_sentence_embedding_for_language"],
            ),
            (
                "sentenceEmbeddingForLanguage:revision:",
                &["nl_sentence_embedding_for_language_revision"],
            ),
            (
                "embeddingWithContentsOfURL:error:",
                &["nl_embedding_with_contents_of_url"],
            ),
            ("containsString:", &["nl_embedding_contains_string"]),
            (
                "distanceBetweenString:andString:distanceType:",
                &["nl_embedding_distance_with_type"],
            ),
            (
                "enumerateNeighborsForString:maximumCount:distanceType:usingBlock:",
                &["enumerate_neighbors_for_string"],
            ),
            (
                "enumerateNeighborsForString:maximumCount:maximumDistance:distanceType:usingBlock:",
                &["neighbors_with_limit", "enumerate_neighbors_for_string"],
            ),
            (
                "neighborsForString:maximumCount:distanceType:",
                &["pub fn neighbors(", "neighbors_with_limit"],
            ),
            (
                "neighborsForString:maximumCount:maximumDistance:distanceType:",
                &["neighbors_with_limit"],
            ),
            (
                "vectorForString:",
                &["nl_embedding_vector_for_string", "pub fn vector_for("],
            ),
            (
                "enumerateNeighborsForVector:maximumCount:distanceType:usingBlock:",
                &["enumerate_neighbors_for_vector"],
            ),
            (
                "enumerateNeighborsForVector:maximumCount:maximumDistance:distanceType:usingBlock:",
                &[
                    "neighbors_for_vector_with_limit",
                    "enumerate_neighbors_for_vector",
                ],
            ),
            (
                "neighborsForVector:maximumCount:distanceType:",
                &[
                    "pub fn neighbors_for_vector(",
                    "neighbors_for_vector_with_limit",
                ],
            ),
            (
                "neighborsForVector:maximumCount:maximumDistance:distanceType:",
                &["neighbors_for_vector_with_limit"],
            ),
            ("dimension", &["nl_embedding_dimension"]),
            ("vocabularySize", &["nl_embedding_vocabulary_size"]),
            ("language", &["nl_embedding_language"]),
            ("revision", &["nl_embedding_revision"]),
            (
                "supportedRevisionsForLanguage:",
                &["nl_embedding_supported_revisions_for_language"],
            ),
            (
                "currentRevisionForLanguage:",
                &["nl_embedding_current_revision_for_language"],
            ),
            (
                "supportedSentenceEmbeddingRevisionsForLanguage:",
                &["nl_embedding_supported_sentence_revisions_for_language"],
            ),
            (
                "currentSentenceEmbeddingRevisionForLanguage:",
                &["nl_embedding_current_sentence_revision_for_language"],
            ),
            (
                "writeEmbeddingForDictionary:language:revision:toURL:error:",
                &["nl_embedding_write_dictionary"],
            ),
        ],
        &omitted_set(["getVector:forString:"]),
    );
}

#[test]
fn nl_gazetteer_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["gazetteer.rs"]),
        bridge_sources(&["GazetteerBridge.swift"]),
    );
    interface_coverage(
        "NLGazetteer",
        "NLGazetteer",
        "NLGazetteer",
        &corpus,
        &[
            (
                "gazetteerWithContentsOfURL:error:",
                &["nl_gazetteer_with_contents_of_url", "pub fn from_path("],
            ),
            (
                "initWithContentsOfURL:error:",
                &["nl_gazetteer_with_contents_of_url", "pub fn from_path("],
            ),
            (
                "initWithData:error:",
                &["nl_gazetteer_with_data", "pub fn from_data("],
            ),
            (
                "initWithDictionary:language:error:",
                &["nl_gazetteer_with_dictionary", "pub fn from_dictionary("],
            ),
            ("labelForString:", &["nl_gazetteer_label_for_string"]),
            ("language", &["nl_gazetteer_language"]),
            ("data", &["nl_gazetteer_data"]),
            (
                "writeGazetteerForDictionary:language:toURL:error:",
                &["nl_gazetteer_write_dictionary"],
            ),
        ],
        &BTreeSet::new(),
    );
}

#[test]
fn nl_model_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["model.rs"]),
        bridge_sources(&["ModelBridge.swift"]),
    );
    interface_coverage(
        "NLModel",
        "NLModel",
        "NLModel",
        &corpus,
        &[
            (
                "modelWithContentsOfURL:error:",
                &["nl_model_with_contents_of_url", "pub fn from_path("],
            ),
            (
                "modelWithMLModel:error:",
                &["nl_model_with_mlmodel", "pub fn from_core_ml_model("],
            ),
            ("configuration", &["nl_model_configuration"]),
            (
                "predictedLabelForString:",
                &["nl_model_predicted_label_for_string"],
            ),
            (
                "predictedLabelsForTokens:",
                &["nl_model_predicted_labels_for_tokens"],
            ),
            (
                "predictedLabelHypothesesForString:maximumCount:",
                &["nl_model_predicted_label_hypotheses_for_string"],
            ),
            (
                "predictedLabelHypothesesForTokens:maximumCount:",
                &["nl_model_predicted_label_hypotheses_for_tokens"],
            ),
        ],
        &BTreeSet::new(),
    );
}

#[test]
fn nl_model_configuration_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["model.rs"]),
        bridge_sources(&["ModelBridge.swift"]),
    );
    interface_coverage(
        "NLModelConfiguration",
        "NLModel",
        "NLModelConfiguration",
        &corpus,
        &[
            ("type", &["nl_model_configuration_type"]),
            ("language", &["nl_model_configuration_language"]),
            ("revision", &["nl_model_configuration_revision"]),
            (
                "supportedRevisionsForType:",
                &["nl_model_supported_revisions_for_type"],
            ),
            (
                "currentRevisionForType:",
                &["nl_model_current_revision_for_type"],
            ),
        ],
        &BTreeSet::new(),
    );
}

#[test]
fn nl_contextual_embedding_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["contextual_embedding.rs"]),
        bridge_sources(&["ContextualEmbeddingBridge.swift"]),
    );
    interface_coverage(
        "NLContextualEmbedding",
        "NLContextualEmbedding",
        "NLContextualEmbedding",
        &corpus,
        &[
            (
                "contextualEmbeddingWithModelIdentifier:",
                &["nl_contextual_embedding_with_model_identifier"],
            ),
            (
                "contextualEmbeddingsForValues:",
                &["nl_contextual_embeddings_for_query"],
            ),
            (
                "contextualEmbeddingWithLanguage:",
                &["nl_contextual_embedding_with_language"],
            ),
            (
                "contextualEmbeddingWithScript:",
                &["nl_contextual_embedding_with_script"],
            ),
            (
                "modelIdentifier",
                &["nl_contextual_embedding_model_identifier"],
            ),
            ("languages", &["nl_contextual_embedding_languages"]),
            ("scripts", &["nl_contextual_embedding_scripts"]),
            ("revision", &["nl_contextual_embedding_revision"]),
            ("dimension", &["nl_contextual_embedding_dimension"]),
            (
                "maximumSequenceLength",
                &["nl_contextual_embedding_maximum_sequence_length"],
            ),
            (
                "loadWithError:",
                &["nl_contextual_embedding_load", "pub fn load("],
            ),
            (
                "unload",
                &["nl_contextual_embedding_unload", "pub fn unload("],
            ),
            (
                "embeddingResultForString:language:error:",
                &["nl_contextual_embedding_result_for_string"],
            ),
            (
                "hasAvailableAssets",
                &["nl_contextual_embedding_has_available_assets"],
            ),
            (
                "requestEmbeddingAssetsWithCompletionHandler:",
                &["nl_contextual_embedding_request_assets"],
            ),
        ],
        &omitted_set(["init"]),
    );
}

#[test]
fn nl_contextual_embedding_result_coverage() {
    let corpus = format!(
        "{}\n{}",
        rust_sources(&["contextual_embedding.rs"]),
        bridge_sources(&["ContextualEmbeddingBridge.swift"]),
    );
    interface_coverage(
        "NLContextualEmbeddingResult",
        "NLContextualEmbedding",
        "NLContextualEmbeddingResult",
        &corpus,
        &[
            ("string", &["nl_contextual_embedding_result_string"]),
            ("language", &["nl_contextual_embedding_result_language"]),
            (
                "sequenceLength",
                &["nl_contextual_embedding_result_sequence_length"],
            ),
            (
                "enumerateTokenVectorsInRange:usingBlock:",
                &["nl_contextual_embedding_result_token_vectors_in_range"],
            ),
            (
                "tokenVectorAtIndex:tokenRange:",
                &["nl_contextual_embedding_result_token_vector_at_index"],
            ),
        ],
        &omitted_set(["init"]),
    );
}

#[test]
fn nl_data_asset_absent_from_current_macos_sdk() {
    let headers_dir =
        sdk_root().join("System/Library/Frameworks/NaturalLanguage.framework/Headers");
    assert!(!headers_dir.join("NLDataAsset.h").exists());

    let mut matches = Vec::new();
    for entry in std::fs::read_dir(&headers_dir).expect("read headers dir") {
        let path = entry.expect("header entry").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("h") {
            continue;
        }
        if read(&path).contains("NLDataAsset") {
            matches.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }

    assert!(
        matches.is_empty(),
        "NLDataAsset unexpectedly present in current NaturalLanguage.framework headers: {matches:?}"
    );
}
