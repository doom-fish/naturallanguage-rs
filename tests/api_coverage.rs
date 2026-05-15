//! API-surface coverage harness for `naturallanguage`.
//!
//! `NaturalLanguage` is an Obj-C framework with proper headers under
//! `NaturalLanguage.framework/Headers/`. Mirror the family pattern
//! (header-based, Obj-C `@interface`, see `speech-rs` / `apple-vision` /
//! `avassetwriter`).

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn read_bridge() -> String {
    read(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "swift-bridge/Sources/NaturalLanguageBridge/NaturalLanguage.swift",
    ))
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

fn extract_member_surface(interface_body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();

    let method_re =
        regex_lite::Regex::new(r"(?m)^\s*[+\-]\s*\([^\)]*\)\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for c in method_re.captures_iter(interface_body) {
        out.insert(c[1].to_string());
    }

    let prop_re = regex_lite::Regex::new(
        r"(?m)^\s*@property\s*(?:\([^\)]*\))?\s*[^;]*?\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:NS_|API_|;)",
    )
    .unwrap();
    for c in prop_re.captures_iter(interface_body) {
        out.insert(c[1].to_string());
    }

    let getter_re = regex_lite::Regex::new(r"getter\s*=\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for c in getter_re.captures_iter(interface_body) {
        out.insert(c[1].to_string());
    }

    out
}

fn references_in_bridge(symbols: &BTreeSet<String>) -> BTreeSet<String> {
    let bridge = read_bridge();
    let aliases = swift_aliases();
    symbols
        .iter()
        .filter(|name| {
            let pattern = format!(r"\b{}\b", regex_lite::escape(name));
            if regex_lite::Regex::new(&pattern).unwrap().is_match(&bridge) {
                return true;
            }
            if let Some(form) = aliases.get(name.as_str()) {
                return bridge.contains(form);
            }
            false
        })
        .cloned()
        .collect()
}

fn swift_aliases() -> std::collections::BTreeMap<&'static str, &'static str> {
    [
        ("initWithUnit", "(unit:"),
        ("initWithTagSchemes", "(tagSchemes:"),
        // Swift drops `For<Noun>` / `With<Noun>` and turns it into the
        // first labelled argument:
        ("dominantLanguageForString", "dominantLanguage(for:"),
        ("languageHypothesesWithMaximum", "languageHypotheses(withMaximum:"),
        ("enumerateTokensInRange", "enumerateTokens(in:"),
        ("enumerateTagsInRange", "enumerateTags("),
    ]
    .into_iter()
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
        .filter(|s| !omitted.contains(*s))
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
        for s in &missing {
            println!("  - {s}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

// ---- Tests ----

#[test]
fn nl_language_recognizer_coverage() {
    let header = read_header("NLLanguageRecognizer");
    let body = extract_interface(&header, "NLLanguageRecognizer");
    let apple = extract_member_surface(&body);
    let ours = references_in_bridge(&apple);
    let omitted = omitted_set([
        // Stateful incremental-feed API; v0.1 single-shot wrapper recreates
        // the recognizer per call so `processString` stays internal.
        "init",
        "reset",
        // Constraints / hints — surface in v0.2 via a builder.
        "languageHints",
        "languageConstraints",
    ]);
    report("NLLanguageRecognizer", &apple, &ours, &omitted);
}

#[test]
fn nl_tokenizer_coverage() {
    let header = read_header("NLTokenizer");
    let body = extract_interface(&header, "NLTokenizer");
    let apple = extract_member_surface(&body);
    let ours = references_in_bridge(&apple);
    let omitted = omitted_set([
        // Random-access cursor + in-place range queries — v0.1 wrapper
        // returns the full token list eagerly so callers iterate in Rust.
        "tokenRangeAtIndex",
        "tokenRangeForRange",
        "tokensForRange",
        // Per-call language override — v0.2 builder.
        "setLanguage",
    ]);
    report("NLTokenizer", &apple, &ours, &omitted);
}

#[test]
fn nl_tagger_coverage() {
    let header = read_header("NLTagger");
    let body = extract_interface(&header, "NLTagger");
    let apple = extract_member_surface(&body);
    let ours = references_in_bridge(&apple);
    let omitted = omitted_set([
        // Random-access tag/range queries; v0.1 returns an eager Vec.
        "tokenRangeAtIndex",
        "tokenRangeForRange",
        "tagAtIndex",
        "tagsInRange",
        "tagHypothesesAtIndex",
        // Other request schemes (lemma, lexicalClass, language detection
        // scheme, sentiment) land in v0.2.
        "availableTagSchemesForUnit",
        "tagSchemes",
        // Per-range orthography / language override — v0.2.
        "setLanguage",
        "setOrthography",
        // Custom NLModel attachments — v0.2.
        "setModels",
        "modelsForTagScheme",
        // NLGazetteer attachments — v0.2.
        "setGazetteers",
        "gazetteersForTagScheme",
        // `dominantLanguage` accessor — covered via `dominant_language()`
        // in the recognizer module instead.
        "dominantLanguage",
        // `requestAssetsForLanguage:tagScheme:completionHandler:` is async
        // model-asset download; v0.1 only ships the always-resident
        // `.nameType` scheme so we don't trigger asset downloads.
        "requestAssetsForLanguage",
    ]);
    report("NLTagger", &apple, &ours, &omitted);
}
