//! Inspect and apply the English contextual embedding (macOS 14+).
//!
//! Run: `cargo run --example 08_contextual_embedding`

use naturallanguage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(embedding) = ContextualEmbedding::for_language(&Language::ENGLISH)? else {
        println!("no contextual embedding available for English");
        return Ok(());
    };

    println!("model identifier = {}", embedding.model_identifier()?);
    println!("languages = {:?}", embedding.languages()?);
    println!("scripts = {:?}", embedding.scripts()?);
    println!("revision = {}", embedding.revision()?);
    println!("dimension = {}", embedding.dimension()?);
    println!("max sequence length = {}", embedding.maximum_sequence_length()?);
    println!("has assets = {}", embedding.has_available_assets()?);

    if let Err(error) = embedding.load() {
        println!("load failed: {error}");
        return Ok(());
    }

    let text = "Hello from Apple in Cupertino.";
    let full = TextRange::new(0, text.encode_utf16().count());
    if let Some(result) = embedding.embedding_result_for_string(text, Some(&Language::ENGLISH))? {
        println!("result language = {:?}", result.language()?);
        println!("sequence length = {}", result.sequence_length());
        println!("full token vectors = {}", result.token_vectors_in_range(full)?.len());
        if let Some(first) = result.token_vector_at_index(0)? {
            println!(
                "first token range = {:?}, dim = {}",
                first.range,
                first.values.len()
            );
        }
    }
    embedding.unload()?;
    Ok(())
}
