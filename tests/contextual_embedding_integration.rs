use std::error::Error;

use naturallanguage::{prelude::*, NLError};

#[test]
fn contextual_embedding_catalogs_and_embeds_when_available() -> Result<(), Box<dyn Error>> {
    let catalog = match ContextualEmbedding::catalog(&ContextualEmbeddingQuery {
        languages: vec![Language::ENGLISH],
        scripts: vec![Script::LATIN],
        revision: None,
    }) {
        Ok(catalog) => catalog,
        Err(NLError::Unsupported(_)) => return Ok(()),
        Err(error) => return Err(error.into()),
    };

    let embedding = match ContextualEmbedding::for_language(&Language::ENGLISH) {
        Ok(Some(embedding)) => embedding,
        Ok(None) | Err(NLError::Unsupported(_)) => return Ok(()),
        Err(error) => return Err(error.into()),
    };

    let model_identifier = embedding.model_identifier()?;
    assert!(!model_identifier.is_empty());
    assert!(catalog.iter().any(|candidate| {
        candidate
            .model_identifier()
            .is_ok_and(|identifier| identifier == model_identifier)
    }));
    assert!(embedding.languages()?.contains(&Language::ENGLISH));
    assert!(embedding.scripts()?.contains(&Script::LATIN));

    let dimension = embedding.dimension()?;
    assert!(dimension > 0);
    assert!(embedding.revision()? > 0);
    assert!(embedding.maximum_sequence_length()? > 0);
    assert!(ContextualEmbedding::from_model_identifier(&model_identifier)?.is_some());

    if !embedding.has_available_assets()? {
        return Ok(());
    }

    match embedding.load() {
        Ok(()) => {}
        Err(NLError::Unsupported(_)) => return Ok(()),
        Err(error) => return Err(error.into()),
    }

    let text = "Hello from Apple in Cupertino.";
    let full = TextRange::new(0, text.encode_utf16().count());
    let result = embedding.embedding_result_for_string(text, Some(&Language::ENGLISH))?;
    let Some(result) = result else {
        embedding.unload()?;
        return Ok(());
    };

    assert_eq!(result.string()?, text);
    assert_eq!(result.language()?, Language::ENGLISH);
    assert!(result.sequence_length() > 0);

    let token_vectors = result.token_vectors_in_range(full)?;
    assert!(!token_vectors.is_empty());
    assert_eq!(token_vectors.len(), result.sequence_length());
    assert!(token_vectors
        .iter()
        .all(|vector| vector.values.len() == dimension));

    let first = result
        .token_vector_at_index(0)?
        .expect("vector at start of string");
    assert_eq!(first.range.start, 0);
    assert_eq!(first.values.len(), dimension);

    embedding.unload()?;
    Ok(())
}
