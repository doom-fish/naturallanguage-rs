use std::error::Error;

use naturallanguage::prelude::*;

#[test]
fn tokenizer_reports_ranges_and_attributes() -> Result<(), Box<dyn Error>> {
    let text = "Hello, world! 👋🏽 There are 42 cats.";
    let full = TextRange::new(0, text.encode_utf16().count());
    let expected = vec!["Hello", "world", "👋🏽", "There", "are", "42", "cats"];

    let eager = tokenize(text, TokenUnit::Word)?;
    assert_eq!(
        eager
            .iter()
            .map(|token| token.text.as_str())
            .collect::<Vec<_>>(),
        expected
    );

    let mut tokenizer = Tokenizer::new(TokenUnit::Word)?;
    tokenizer.set_string(Some(text))?;
    tokenizer.set_language(&Language::ENGLISH)?;

    assert_eq!(tokenizer.unit(), TokenUnit::Word);
    assert_eq!(tokenizer.string()?, Some(text.to_string()));
    assert_eq!(tokenizer.token_range_at_index(1)?, TextRange::new(0, 5));
    assert_eq!(
        tokenizer.token_range_for_range(TextRange::new(0, 5))?,
        TextRange::new(0, 5)
    );

    let tokens = tokenizer.tokens_in_range(full)?;
    assert_eq!(tokens.len(), expected.len());
    assert_eq!(
        tokenizer.token_ranges_for_range(full)?.len(),
        expected.len()
    );

    let emoji = tokens
        .iter()
        .find(|token| token.text == "👋🏽")
        .expect("emoji token");
    assert!(emoji.attributes.contains(TokenizerAttributes::EMOJI));

    let numeric = tokens
        .iter()
        .find(|token| token.text == "42")
        .expect("numeric token");
    assert!(numeric.attributes.contains(TokenizerAttributes::NUMERIC));
    Ok(())
}
