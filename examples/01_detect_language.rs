//! Detect dominant language + multi-hypothesis ranking on a few sample
//! strings.
//!
//! Run: `cargo run --example 01_detect_language`

use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let samples = [
        "The quick brown fox jumps over the lazy dog.",
        "Bonjour le monde, comment ça va aujourd'hui?",
        "Ich habe heute ein neues Auto gekauft.",
        "Hejsan! Hur mår du? Vad gör du i kväll?",
        "你好，世界。今天天气真不错。",
        "今日はいい天気ですね。",
    ];

    println!("== dominant language ==");
    for s in &samples {
        let lang = dominant_language(s)?;
        println!("  {lang:?}  ←  {:?}", &s[..s.char_indices().nth(40).map_or(s.len(), |(i, _)| i)]);
    }

    println!("\n== multi-hypothesis (top 3) ==");
    for s in &samples {
        let hyps = language_hypotheses(s, 3)?;
        let summary = hyps
            .iter()
            .map(|h| format!("{}={:.2}", h.language, h.confidence))
            .collect::<Vec<_>>()
            .join(", ");
        println!("  [{summary}]  ←  {:?}", &s[..s.char_indices().nth(40).map_or(s.len(), |(i, _)| i)]);
    }
    Ok(())
}
