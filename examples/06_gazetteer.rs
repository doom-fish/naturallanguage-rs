//! Build a custom gazetteer, round-trip it to disk, and attach it to `Tagger`.
//!
//! Run: `cargo run --example 06_gazetteer`

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dictionary = BTreeMap::new();
    dictionary.insert(Tag::PERSONAL_NAME.as_str().to_string(), vec!["Ada Lovelace".to_string()]);
    dictionary.insert(Tag::PLACE_NAME.as_str().to_string(), vec!["Gotham".to_string()]);
    dictionary.insert(Tag::ORGANIZATION_NAME.as_str().to_string(), vec!["Acme Labs".to_string()]);

    let gazetteer = Gazetteer::from_dictionary(&dictionary, Some(&Language::ENGLISH))?;
    println!("language = {:?}", gazetteer.language()?);
    println!("label(Acme Labs) = {:?}", gazetteer.label_for_string("Acme Labs")?);
    println!("serialized bytes = {}", gazetteer.data()?.len());

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-output");
    fs::create_dir_all(&output_dir)?;
    let gazetteer_path = output_dir.join("custom_names.gazetteer");
    Gazetteer::write_dictionary(&dictionary, Some(&Language::ENGLISH), &gazetteer_path)?;
    let roundtrip = Gazetteer::from_path(&gazetteer_path)?;
    println!("roundtrip label(Gotham) = {:?}", roundtrip.label_for_string("Gotham")?);

    let text = "Ada Lovelace joined Acme Labs in Gotham.";
    let full = TextRange::new(0, text.encode_utf16().count());
    let mut tagger = Tagger::new(&[TagScheme::NAME_TYPE])?;
    tagger.set_string(Some(text))?;
    tagger.set_gazetteers(&[&roundtrip], &TagScheme::NAME_TYPE)?;
    println!("attached gazetteers = {}", tagger.gazetteers_for_tag_scheme(&TagScheme::NAME_TYPE)?.len());
    for tagged in tagger.tags_in_range(
        full,
        TokenUnit::Word,
        &TagScheme::NAME_TYPE,
        TaggerOptions::OMIT_PUNCTUATION
            | TaggerOptions::OMIT_WHITESPACE
            | TaggerOptions::OMIT_OTHER
            | TaggerOptions::JOIN_NAMES,
    )? {
        if tagged.tag == Some(Tag::OTHER_WORD) {
            continue;
        }
        println!("  {:?} -> {:?}", tagged.text, tagged.tag);
    }
    Ok(())
}
