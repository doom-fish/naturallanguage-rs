use std::error::Error;

use naturallanguage::prelude::*;

#[test]
fn language_recognizer_tracks_hints_constraints_and_reset() -> Result<(), Box<dyn Error>> {
    let english_text = "This is a short English sentence about Apple and Rust.";
    assert_eq!(dominant_language(english_text)?, Some("en".to_string()));

    let hypotheses = language_hypotheses(english_text, 3)?;
    assert!(!hypotheses.is_empty());
    assert_eq!(hypotheses[0].language, Language::ENGLISH.as_str());

    let mut recognizer = LanguageRecognizer::new()?;
    recognizer.set_language_hints(&[
        LanguageHypothesis {
            language: Language::ENGLISH.as_str().to_string(),
            confidence: 0.8,
        },
        LanguageHypothesis {
            language: Language::GERMAN.as_str().to_string(),
            confidence: 0.2,
        },
    ])?;
    recognizer.set_language_constraints(&[Language::ENGLISH, Language::GERMAN])?;

    let hints = recognizer.language_hints()?;
    assert_eq!(hints.len(), 2);
    assert!(hints
        .iter()
        .any(|hint| hint.language == Language::ENGLISH.as_str()));
    assert!(hints
        .iter()
        .any(|hint| hint.language == Language::GERMAN.as_str()));

    let constraints = recognizer.language_constraints()?;
    assert!(constraints.contains(&Language::ENGLISH));
    assert!(constraints.contains(&Language::GERMAN));

    recognizer.process(english_text)?;
    assert_eq!(recognizer.dominant_language()?, Some(Language::ENGLISH));
    let recognizer_hypotheses = recognizer.language_hypotheses(2)?;
    assert_eq!(
        recognizer_hypotheses[0].language,
        Language::ENGLISH.as_str()
    );

    recognizer.reset();
    recognizer.process("Das ist ein kurzer deutscher Satz.")?;
    assert_eq!(recognizer.dominant_language()?, Some(Language::GERMAN));
    Ok(())
}
