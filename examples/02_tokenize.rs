//! Tokenise a sample paragraph at word + sentence + paragraph granularity.
//!
//! Run: `cargo run --example 02_tokenize`

use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello, world! This is a small paragraph. \
                It has three sentences. Each one is short.\n\n\
                Here's a second paragraph for the document-level test.";

    println!("== words ==");
    for t in tokenize(text, TokenUnit::Word)? {
        println!("  [{:>3}+{:>2}] {:?}", t.start, t.length, t.text);
    }

    println!("\n== sentences ==");
    for t in tokenize(text, TokenUnit::Sentence)? {
        println!("  [{:>3}+{:>3}] {:?}", t.start, t.length, t.text);
    }

    println!("\n== paragraphs ==");
    for t in tokenize(text, TokenUnit::Paragraph)? {
        println!("  [{:>3}+{:>3}] {:?}", t.start, t.length, t.text);
    }
    Ok(())
}
