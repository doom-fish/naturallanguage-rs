#![cfg(feature = "tag")]

use naturallanguage::prelude::*;

#[test]
fn tagger_asset_request_completes_with_a_result() {
    let result = Tagger::request_assets(&Language::ENGLISH, &TagScheme::LEXICAL_CLASS);
    assert!(result.is_ok(), "{result:?}");
}
