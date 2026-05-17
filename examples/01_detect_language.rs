//! Detect dominant language using both the convenience functions and the
//! stateful `LanguageRecognizer` API.
//!
//! Run: `cargo run --example 01_detect_language`

use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Bonjour! Tim Cook visited Paris and spoke a little English.";

    println!("single-shot dominant = {:?}", dominant_language(text)?);
    println!(
        "single-shot hypotheses = {:?}",
        language_hypotheses(text, 3)?
    );

    let mut recognizer = LanguageRecognizer::new()?;
    recognizer.set_language_hints(&[
        LanguageHypothesis {
            language: Language::FRENCH.as_str().to_string(),
            confidence: 0.7,
        },
        LanguageHypothesis {
            language: Language::ENGLISH.as_str().to_string(),
            confidence: 0.3,
        },
    ])?;
    recognizer.set_language_constraints(&[
        Language::FRENCH,
        Language::ENGLISH,
        Language::GERMAN,
    ])?;
    recognizer.process(text)?;

    println!(
        "recognizer dominant = {:?}",
        recognizer.dominant_language()?
    );
    println!("recognizer hints = {:?}", recognizer.language_hints()?);
    println!(
        "recognizer constraints = {:?}",
        recognizer.language_constraints()?
    );
    println!(
        "recognizer hypotheses = {:?}",
        recognizer.language_hypotheses(3)?
    );

    recognizer.reset();
    recognizer.process("Das ist ein kurzer deutscher Satz.")?;
    println!(
        "after reset dominant = {:?}",
        recognizer.dominant_language()?
    );
    Ok(())
}
