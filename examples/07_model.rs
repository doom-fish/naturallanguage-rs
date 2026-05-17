//! Load custom Core ML text models via `NLModel`.
//!
//! Run: `cargo run --example 07_model`

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use naturallanguage::prelude::*;

fn compile_model(source: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-output");
    fs::create_dir_all(&output_dir)?;
    let compiled = output_dir.join(format!(
        "{}.mlmodelc",
        source.file_stem().unwrap().to_string_lossy()
    ));
    if compiled.exists() {
        fs::remove_dir_all(&compiled)?;
    }
    let status = Command::new("xcrun")
        .args([
            "coremlcompiler",
            "compile",
            source.to_str().unwrap(),
            output_dir.to_str().unwrap(),
        ])
        .status()?;
    if !status.success() {
        return Err(format!("failed to compile {}", source.display()).into());
    }
    Ok(compiled)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/assets");
    let classifier_path = assets.join("sentiment_classifier.mlmodel");
    let sequence_path = assets.join("name_tagger.mlmodel");
    let compiled_classifier = compile_model(&classifier_path)?;
    let compiled_sequence = compile_model(&sequence_path)?;

    let coreml = CoreMlModel::from_source_path(&classifier_path)?;
    let classifier = Model::from_core_ml_model(&coreml)?;
    let classifier_from_path = Model::from_path(&compiled_classifier)?;
    let classifier_config = classifier.configuration()?;
    println!(
        "classifier config: type={:?} language={:?} revision={}",
        classifier_config.model_type(),
        classifier_config.language()?,
        classifier_config.revision()
    );
    println!(
        "classifier supported revisions = {:?}",
        ModelConfiguration::supported_revisions_for_type(ModelType::Classifier)?
    );
    println!(
        "classifier current revision = {}",
        ModelConfiguration::current_revision_for_type(ModelType::Classifier)
    );
    println!(
        "path-loaded classifier label = {:?}",
        classifier_from_path.predicted_label_for_string("This is fantastic")?
    );
    for sample in ["I love this product", "This is terrible"] {
        println!(
            "  {sample:?} -> {:?}",
            classifier.predicted_label_for_string(sample)?
        );
        println!(
            "    hypotheses = {:?}",
            classifier.predicted_label_hypotheses_for_string(sample, 2)?
        );
    }

    let sequence = Model::from_path(&compiled_sequence)?;
    let sequence_config = sequence.configuration()?;
    let tokens = vec![
        "Tim".to_string(),
        "Cook".to_string(),
        "visited".to_string(),
        "Cupertino".to_string(),
    ];
    println!(
        "sequence config: type={:?} language={:?} revision={}",
        sequence_config.model_type(),
        sequence_config.language()?,
        sequence_config.revision()
    );
    println!(
        "sequence labels = {:?}",
        sequence.predicted_labels_for_tokens(&tokens)?
    );
    println!(
        "sequence hypotheses = {:?}",
        sequence.predicted_label_hypotheses_for_tokens(&tokens, 2)?
    );
    Ok(())
}
