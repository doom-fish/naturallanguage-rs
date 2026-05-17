//! Tokenise text with the eager helper and the stateful `Tokenizer` API.
//!
//! Run: `cargo run --example 02_tokenize`

use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello, world! 👋🏽 There are 42 cats.";
    let full = TextRange::new(0, text.encode_utf16().count());

    println!("== eager helper ==");
    for token in tokenize(text, TokenUnit::Word)? {
        println!(
            "  [{:>3}+{:>2}] {:?}",
            token.start, token.length, token.text
        );
    }

    let mut tokenizer = Tokenizer::new(TokenUnit::Word)?;
    tokenizer.set_string(Some(text))?;
    tokenizer.set_language(&Language::ENGLISH)?;

    println!("\nunit = {:?}", tokenizer.unit());
    println!(
        "token at utf16 index 1 = {:?}",
        tokenizer.token_range_at_index(1)?
    );
    println!(
        "token range covering [0+5] = {:?}",
        tokenizer.token_range_for_range(TextRange::new(0, 5))?
    );

    println!("\n== object API tokens ==");
    for token in tokenizer.tokens_in_range(full)? {
        println!(
            "  [{:>3}+{:>2}] {:?} numeric={} symbolic={} emoji={}",
            token.range.start,
            token.range.length,
            token.text,
            token.attributes.contains(TokenizerAttributes::NUMERIC),
            token.attributes.contains(TokenizerAttributes::SYMBOLIC),
            token.attributes.contains(TokenizerAttributes::EMOJI)
        );
    }
    Ok(())
}
