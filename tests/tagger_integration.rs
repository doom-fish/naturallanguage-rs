use std::error::Error;

use naturallanguage::prelude::*;

#[test]
fn tagger_supports_scheme_lookup_and_tag_queries() -> Result<(), Box<dyn Error>> {
    let text = "Tim Cook visited Apple Park in Cupertino.";
    let full = TextRange::new(0, text.encode_utf16().count());

    let schemes = Tagger::available_tag_schemes(TokenUnit::Word, &Language::ENGLISH)?;
    assert!(schemes.contains(&TagScheme::LEXICAL_CLASS));
    assert!(schemes.contains(&TagScheme::NAME_TYPE));

    let mut tagger = Tagger::new(&[
        TagScheme::LEXICAL_CLASS,
        TagScheme::LEMMA,
        TagScheme::LANGUAGE,
        TagScheme::NAME_TYPE,
    ])?;
    tagger.set_string(Some(text))?;
    tagger.set_language(&Language::ENGLISH, full)?;

    let mut orthography = Orthography::new().with_dominant_script(Script::LATIN);
    orthography.insert(Script::LATIN, vec![Language::ENGLISH]);
    tagger.set_orthography(&orthography, full)?;

    assert_eq!(tagger.dominant_language()?, Some(Language::ENGLISH));

    let first = tagger.tag_at_index(0, TokenUnit::Word, &TagScheme::LEXICAL_CLASS)?;
    assert_eq!(first.text, "Tim");
    assert_eq!(
        first.tag.as_ref().map(Tag::as_str),
        Some(Tag::NOUN.as_str())
    );

    let (range, hypotheses) =
        tagger.tag_hypotheses_at_index(0, TokenUnit::Word, &TagScheme::LEXICAL_CLASS, 3)?;
    assert_eq!(range, TextRange::new(0, 3));
    assert_eq!(
        hypotheses.first().map(|hypothesis| hypothesis.tag.as_str()),
        Some(Tag::NOUN.as_str())
    );

    let name_tags = tagger.tags_in_range(
        full,
        TokenUnit::Word,
        &TagScheme::NAME_TYPE,
        TaggerOptions::OMIT_PUNCTUATION
            | TaggerOptions::OMIT_WHITESPACE
            | TaggerOptions::OMIT_OTHER,
    )?;
    assert!(name_tags.iter().any(|tagged| {
        tagged.text == "Tim"
            && tagged.tag.as_ref().map(Tag::as_str) == Some(Tag::PERSONAL_NAME.as_str())
    }));
    assert!(name_tags.iter().any(|tagged| {
        tagged.text == "Cook"
            && tagged.tag.as_ref().map(Tag::as_str) == Some(Tag::PERSONAL_NAME.as_str())
    }));
    assert!(name_tags.iter().any(|tagged| {
        tagged.text == "Cupertino"
            && tagged.tag.as_ref().map(Tag::as_str) == Some(Tag::PLACE_NAME.as_str())
    }));

    let lemmas = tagger.tags_in_range(
        full,
        TokenUnit::Word,
        &TagScheme::LEMMA,
        TaggerOptions::OMIT_PUNCTUATION | TaggerOptions::OMIT_WHITESPACE,
    )?;
    assert!(lemmas.iter().any(|tagged| {
        tagged.text == "visited" && tagged.tag.as_ref().map(Tag::as_str) == Some("visit")
    }));
    Ok(())
}
