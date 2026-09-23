#![cfg(feature = "model")]

mod common;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use naturallanguage::prelude::*;

fn compiled_copies(stem: &str) -> Vec<PathBuf> {
    fs::read_dir(std::env::temp_dir())
        .expect("read temporary directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(stem) && name.ends_with(".mlmodelc"))
        })
        .collect()
}

#[test]
#[ignore = "Core ML compiles a .mlmodel source into the system temporary directory, outside target/"]
fn compiled_source_models_are_removed_with_the_last_reference() -> Result<(), Box<dyn Error>> {
    let stem = format!("nl_cleanup_probe_{}_", std::process::id());
    let source = common::artifact_path("model_cleanup", &format!("{stem}.mlmodel"));
    fs::copy(common::asset_path("sentiment_classifier.mlmodel"), &source)?;
    assert!(compiled_copies(&stem).is_empty());

    let coreml = CoreMlModel::from_source_path(&source)?;
    assert_eq!(compiled_copies(&stem).len(), 1);
    let model = Model::from_core_ml_model(&coreml)?;
    drop(coreml);
    assert_eq!(
        model.predicted_label_for_string("I love this product")?,
        Some("positive".to_string())
    );
    drop(model);
    assert!(compiled_copies(&stem).is_empty());

    drop(CoreMlModel::from_source_path(&source)?);
    assert!(compiled_copies(&stem).is_empty());
    fs::remove_file(&source)?;
    Ok(())
}
