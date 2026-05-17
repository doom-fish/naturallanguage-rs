mod common;

use std::error::Error;

use naturallanguage::prelude::*;

#[test]
fn model_loads_classifier_and_sequence_predictions() -> Result<(), Box<dyn Error>> {
    let classifier_source = common::asset_path("sentiment_classifier.mlmodel");
    let sequence_source = common::asset_path("name_tagger.mlmodel");
    let compiled_classifier = common::compile_model(&classifier_source, "model");
    let compiled_sequence = common::compile_model(&sequence_source, "model");

    let coreml_source = CoreMlModel::from_source_path(&classifier_source)?;
    let _coreml_compiled = CoreMlModel::from_compiled_path(&compiled_classifier)?;

    let classifier = Model::from_core_ml_model(&coreml_source)?;
    let classifier_from_path = Model::from_path(&compiled_classifier)?;
    let classifier_config = classifier.configuration()?;
    assert_eq!(classifier_config.model_type(), ModelType::Classifier);
    assert_eq!(
        classifier_config.revision(),
        ModelConfiguration::current_revision_for_type(ModelType::Classifier)
    );
    assert!(
        ModelConfiguration::supported_revisions_for_type(ModelType::Classifier)?
            .contains(&classifier_config.revision())
    );

    assert_eq!(
        classifier.predicted_label_for_string("I love this product")?,
        Some("positive".to_string())
    );
    assert_eq!(
        classifier.predicted_label_for_string("This is terrible")?,
        Some("negative".to_string())
    );
    assert_eq!(
        classifier_from_path.predicted_label_for_string("This is terrible")?,
        Some("negative".to_string())
    );

    let classifier_hypotheses =
        classifier.predicted_label_hypotheses_for_string("This is terrible", 2)?;
    assert_eq!(classifier_hypotheses.len(), 2);
    assert_eq!(classifier_hypotheses[0].0, "negative");
    assert!(classifier_hypotheses[0].1 >= classifier_hypotheses[1].1);

    let sequence = Model::from_path(&compiled_sequence)?;
    let sequence_config = sequence.configuration()?;
    assert_eq!(sequence_config.model_type(), ModelType::Sequence);

    let tokens = vec![
        "Tim".to_string(),
        "Cook".to_string(),
        "visited".to_string(),
        "Cupertino".to_string(),
    ];
    assert_eq!(
        sequence.predicted_labels_for_tokens(&tokens)?,
        vec![
            "ORG".to_string(),
            "ORG".to_string(),
            "O".to_string(),
            "ORG".to_string(),
        ]
    );

    let sequence_hypotheses = sequence.predicted_label_hypotheses_for_tokens(&tokens, 2)?;
    assert_eq!(sequence_hypotheses.len(), tokens.len());
    assert!(sequence_hypotheses
        .iter()
        .all(|hypotheses| !hypotheses.is_empty()));
    Ok(())
}
