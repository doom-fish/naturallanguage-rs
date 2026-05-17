mod common;

use std::collections::BTreeMap;
use std::error::Error;

use naturallanguage::prelude::*;

#[test]
fn gazetteer_roundtrips_and_tags_custom_names() -> Result<(), Box<dyn Error>> {
    let mut dictionary = BTreeMap::new();
    dictionary.insert(
        Tag::PERSONAL_NAME.as_str().to_string(),
        vec!["Ada Lovelace".to_string()],
    );
    dictionary.insert(
        Tag::PLACE_NAME.as_str().to_string(),
        vec!["Gotham".to_string()],
    );
    dictionary.insert(
        Tag::ORGANIZATION_NAME.as_str().to_string(),
        vec!["Acme Labs".to_string()],
    );

    let gazetteer = Gazetteer::from_dictionary(&dictionary, Some(&Language::ENGLISH))?;
    assert_eq!(
        gazetteer.label_for_string("Ada Lovelace")?,
        Some(Tag::PERSONAL_NAME.as_str().to_string())
    );
    assert_eq!(
        gazetteer.label_for_string("Acme Labs")?,
        Some(Tag::ORGANIZATION_NAME.as_str().to_string())
    );

    let data = gazetteer.data()?;
    assert!(!data.is_empty());
    let from_data = Gazetteer::from_data(&data)?;
    assert_eq!(
        from_data.label_for_string("Gotham")?,
        Some(Tag::PLACE_NAME.as_str().to_string())
    );

    let path = common::artifact_path("gazetteer", "custom_names.gazetteer");
    Gazetteer::write_dictionary(&dictionary, Some(&Language::ENGLISH), &path)?;
    let roundtrip = Gazetteer::from_path(&path)?;
    assert_eq!(
        roundtrip.label_for_string("Acme Labs")?,
        Some(Tag::ORGANIZATION_NAME.as_str().to_string())
    );

    let text = "Ada Lovelace joined Acme Labs in Gotham.";
    let full = TextRange::new(0, text.encode_utf16().count());
    let mut tagger = Tagger::new(&[TagScheme::NAME_TYPE])?;
    tagger.set_string(Some(text))?;
    tagger.set_gazetteers(&[&roundtrip], &TagScheme::NAME_TYPE)?;
    assert_eq!(
        tagger
            .gazetteers_for_tag_scheme(&TagScheme::NAME_TYPE)?
            .len(),
        1
    );

    let tags = tagger.tags_in_range(
        full,
        TokenUnit::Word,
        &TagScheme::NAME_TYPE,
        TaggerOptions::OMIT_PUNCTUATION
            | TaggerOptions::OMIT_WHITESPACE
            | TaggerOptions::OMIT_OTHER
            | TaggerOptions::JOIN_NAMES,
    )?;
    assert!(tags.iter().any(|tagged| {
        tagged.text == "Ada Lovelace"
            && tagged.tag.as_ref().map(Tag::as_str) == Some(Tag::PERSONAL_NAME.as_str())
    }));
    assert!(tags.iter().any(|tagged| {
        tagged.text == "Acme Labs"
            && tagged.tag.as_ref().map(Tag::as_str) == Some(Tag::ORGANIZATION_NAME.as_str())
    }));
    assert!(tags.iter().any(|tagged| {
        tagged.text == "Gotham"
            && tagged.tag.as_ref().map(Tag::as_str) == Some(Tag::PLACE_NAME.as_str())
    }));
    Ok(())
}
