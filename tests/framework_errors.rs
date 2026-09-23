#![cfg(feature = "model")]

use naturallanguage::prelude::*;

#[test]
fn a_framework_error_is_reported_as_unknown_with_its_message() {
    let result = Model::from_path("/nonexistent/naturallanguage-rs/missing.mlmodelc");
    match result {
        Err(NLError::Unknown { message, .. }) => assert!(message.contains("not found")),
        other => panic!("expected a framework error, got {other:?}"),
    }
}
