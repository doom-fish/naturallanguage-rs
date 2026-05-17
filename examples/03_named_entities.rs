//! Extract named entities (persons, places, organisations) from a few
//! sample sentences.
//!
//! Run: `cargo run --example 03_named_entities`

use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let samples = [
        "Tim Cook visited Apple Park in Cupertino last Tuesday.",
        "Barack Obama and Joe Biden met with Angela Merkel in Berlin.",
        "The European Central Bank moved its headquarters from Brussels to Frankfurt.",
        "Steve Jobs co-founded Apple Computer with Steve Wozniak in 1976.",
    ];

    for s in &samples {
        println!("\n{s:?}");
        let entities = named_entities(s)?;
        if entities.is_empty() {
            println!("  (no named entities found)");
        }
        for e in &entities {
            println!(
                "  {:>16?}: [{:>3}+{:>2}] {:?}",
                e.kind, e.start, e.length, e.text
            );
        }
    }
    Ok(())
}
