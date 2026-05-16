//! Run lexical-class, lemma, language, script, and name-type tagging.
//!
//! Run: `cargo run --example 05_tagger`

use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Tim Cook visited Apple Park in Cupertino.";
    let full = TextRange::new(0, text.encode_utf16().count());

    println!(
        "available English word schemes = {:?}",
        Tagger::available_tag_schemes(TokenUnit::Word, &Language::ENGLISH)?
    );

    let mut tagger = Tagger::new(&[
        TagScheme::LEXICAL_CLASS,
        TagScheme::LEMMA,
        TagScheme::LANGUAGE,
        TagScheme::SCRIPT,
        TagScheme::NAME_TYPE,
    ])?;
    tagger.set_string(Some(text))?;
    tagger.set_language(&Language::ENGLISH, full)?;

    let mut orthography = Orthography::new().with_dominant_script(Script::LATIN);
    orthography.insert(Script::LATIN, vec![Language::ENGLISH]);
    tagger.set_orthography(&orthography, full)?;

    println!("dominant language = {:?}", tagger.dominant_language()?);
    println!("tag schemes = {:?}", tagger.tag_schemes()?);

    for (label, scheme) in [
        ("lexical", TagScheme::LEXICAL_CLASS),
        ("lemma", TagScheme::LEMMA),
        ("language", TagScheme::LANGUAGE),
        ("script", TagScheme::SCRIPT),
        ("names", TagScheme::NAME_TYPE),
    ] {
        println!("\n== {label} ==");
        for tagged in tagger.tags_in_range(
            full,
            TokenUnit::Word,
            &scheme,
            TaggerOptions::OMIT_PUNCTUATION
                | TaggerOptions::OMIT_WHITESPACE
                | if scheme == TagScheme::NAME_TYPE {
                    TaggerOptions::OMIT_OTHER
                } else {
                    TaggerOptions::NONE
                },
        )? {
            if scheme == TagScheme::NAME_TYPE && tagged.tag == Some(Tag::OTHER_WORD) {
                continue;
            }
            println!("  {:?} -> {:?}", tagged.text, tagged.tag);
        }
    }

    let first = tagger.tag_at_index(0, TokenUnit::Word, &TagScheme::LEXICAL_CLASS)?;
    let (range, hypotheses) =
        tagger.tag_hypotheses_at_index(0, TokenUnit::Word, &TagScheme::LEXICAL_CLASS, 3)?;
    println!("\nfirst lexical tag = {first:?}");
    println!("first lexical hypotheses @ {range:?} = {hypotheses:?}");
    Ok(())
}
