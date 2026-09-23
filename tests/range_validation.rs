#![cfg(all(feature = "tag", feature = "contextual_embedding"))]

use std::collections::BTreeMap;

use naturallanguage::prelude::*;

const TEXT: &str = "Hello brave new world";

fn hostile_ranges() -> Vec<TextRange> {
    let utf16_len = TEXT.encode_utf16().count();
    vec![
        TextRange::new(1, usize::MAX),
        TextRange::new(5, isize::MAX.unsigned_abs()),
        TextRange::new(isize::MAX.unsigned_abs(), 1),
        TextRange::new(usize::MAX, 0),
        TextRange::new(0, utf16_len + 1),
        TextRange::new(utf16_len + 1, 0),
    ]
}

fn assert_invalid<T: std::fmt::Debug>(result: &Result<T, NLError>, what: &str, range: TextRange) {
    assert!(
        matches!(result, Err(NLError::InvalidArgument(_))),
        "{what} accepted {range:?}: {result:?}"
    );
}

fn tagger() -> Tagger {
    let mut tagger = Tagger::new(&[TagScheme::LEXICAL_CLASS]).expect("tagger");
    tagger.set_string(Some(TEXT)).expect("set string");
    tagger
}

#[test]
fn text_range_end_saturates() {
    assert_eq!(TextRange::new(1, usize::MAX).end(), usize::MAX);
    assert_eq!(TextRange::new(2, 3).end(), 5);
}

#[test]
fn tagger_rejects_hostile_ranges() {
    let mut tagger = tagger();
    let orthography = Orthography {
        dominant_script: Some(Script::LATIN),
        language_map: BTreeMap::from([(Script::LATIN, vec![Language::ENGLISH])]),
    };
    for range in hostile_ranges() {
        assert_invalid(
            &tagger.token_range_for_range(range, TokenUnit::Word),
            "token_range_for_range",
            range,
        );
        assert_invalid(
            &tagger.tags_in_range(
                range,
                TokenUnit::Word,
                &TagScheme::LEXICAL_CLASS,
                TaggerOptions::NONE,
            ),
            "tags_in_range",
            range,
        );
        assert_invalid(
            &tagger.set_language(&Language::ENGLISH, range),
            "set_language",
            range,
        );
        assert_invalid(
            &tagger.set_orthography(&orthography, range),
            "set_orthography",
            range,
        );
    }
    for index in [
        usize::MAX,
        isize::MAX.unsigned_abs(),
        TEXT.encode_utf16().count(),
    ] {
        assert!(tagger.token_range_at_index(index, TokenUnit::Word).is_err());
        assert!(tagger
            .tag_at_index(index, TokenUnit::Word, &TagScheme::LEXICAL_CLASS)
            .is_err());
        assert!(tagger
            .tag_hypotheses_at_index(index, TokenUnit::Word, &TagScheme::LEXICAL_CLASS, 3)
            .is_err());
    }
    let full = TextRange::new(0, TEXT.encode_utf16().count());
    assert_eq!(
        tagger.token_range_for_range(TextRange::new(1, 2), TokenUnit::Word),
        Ok(TextRange::new(0, 5))
    );
    tagger
        .set_language(&Language::ENGLISH, full)
        .expect("valid set_language");
    tagger
        .set_orthography(&orthography, full)
        .expect("valid set_orthography");
    let tags = tagger
        .tags_in_range(
            full,
            TokenUnit::Word,
            &TagScheme::LEXICAL_CLASS,
            TaggerOptions::OMIT_WHITESPACE,
        )
        .expect("valid tags_in_range");
    assert_eq!(tags.len(), 4);
}

#[test]
fn tokenizer_rejects_hostile_ranges() {
    let mut tokenizer = Tokenizer::new(TokenUnit::Word).expect("tokenizer");
    tokenizer.set_string(Some(TEXT)).expect("set string");
    for range in hostile_ranges() {
        assert_invalid(&tokenizer.tokens_in_range(range), "tokens_in_range", range);
        assert_invalid(
            &tokenizer.token_range_for_range(range),
            "token_range_for_range",
            range,
        );
    }
    for index in [
        usize::MAX,
        isize::MAX.unsigned_abs(),
        TEXT.encode_utf16().count(),
    ] {
        assert!(tokenizer.token_range_at_index(index).is_err());
    }
    let full = TextRange::new(0, TEXT.encode_utf16().count());
    assert_eq!(tokenizer.tokens_in_range(full).expect("tokens").len(), 4);
}

#[test]
fn contextual_embedding_rejects_hostile_ranges() -> Result<(), NLError> {
    let embedding = match ContextualEmbedding::for_language(&Language::ENGLISH) {
        Ok(Some(embedding)) => embedding,
        Ok(None) | Err(NLError::Unsupported(_)) => return Ok(()),
        Err(error) => return Err(error),
    };
    if !embedding.has_available_assets()? {
        return Ok(());
    }
    embedding.load()?;
    let Some(result) = embedding.embedding_result_for_string(TEXT, Some(&Language::ENGLISH))?
    else {
        return Ok(());
    };
    for range in hostile_ranges() {
        assert_invalid(
            &result.token_vectors_in_range(range),
            "token_vectors_in_range",
            range,
        );
    }
    for index in [
        usize::MAX,
        isize::MAX.unsigned_abs(),
        TEXT.encode_utf16().count(),
    ] {
        assert!(result.token_vector_at_index(index).is_err());
    }
    let full = TextRange::new(0, TEXT.encode_utf16().count());
    assert!(!result.token_vectors_in_range(full)?.is_empty());
    embedding.unload()?;
    Ok(())
}
