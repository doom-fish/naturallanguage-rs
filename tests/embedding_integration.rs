use std::error::Error;

use naturallanguage::prelude::*;

#[test]
fn embedding_exposes_vectors_distances_and_revisions() -> Result<(), Box<dyn Error>> {
    let embedding = Embedding::word_for_language(Language::ENGLISH.as_str())?
        .expect("expected built-in English word embedding");

    assert!(embedding.dimension() > 0);
    assert!(embedding.vocabulary_size() > 10_000);

    let supported_revisions = Embedding::supported_revisions_for_language(&Language::ENGLISH)?;
    assert!(supported_revisions.contains(&embedding.revision()));
    assert_eq!(
        Embedding::current_revision_for_language(&Language::ENGLISH),
        embedding.revision()
    );

    assert!(embedding.contains_string("king")?);
    let king_vector = embedding.vector_for("king")?.expect("vector for king");
    assert_eq!(king_vector.len(), embedding.dimension());

    let king_queen = embedding
        .distance("king", "queen")?
        .expect("king/queen distance");
    let king_pizza = embedding
        .distance_with_type("king", "pizza", DistanceType::Cosine)?
        .expect("king/pizza distance");
    assert!(king_queen < king_pizza);

    let neighbors = embedding.neighbors("computer", 5)?;
    assert_eq!(neighbors.len(), 5);
    assert!(neighbors
        .iter()
        .all(|neighbor| !neighbor.word.is_empty() && neighbor.distance >= 0.0));

    let vector_neighbors = embedding.neighbors_for_vector(&king_vector, 3, DistanceType::Cosine)?;
    assert!(!vector_neighbors.is_empty());
    Ok(())
}
