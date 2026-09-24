#![cfg(feature = "tag")]

mod common;

use naturallanguage::prelude::*;

#[test]
fn tagger_asset_request_completes_with_a_result() {
    if !common::live_tests_enabled("tagger_asset_request_completes_with_a_result") {
        return;
    }
    let result = Tagger::request_assets(&Language::ENGLISH, &TagScheme::LEXICAL_CLASS);
    assert!(result.is_ok(), "{result:?}");
}
