//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

use crate::types::TextRange;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TextRangeRaw {
    pub start: usize,
    pub length: usize,
}

impl From<TextRange> for TextRangeRaw {
    fn from(value: TextRange) -> Self {
        Self {
            start: value.start,
            length: value.length,
        }
    }
}

#[repr(C)]
pub struct LanguageHypothesisRaw {
    pub language: *mut c_char,
    pub confidence: f64,
}

#[repr(C)]
pub struct LanguageHypothesisRefRaw {
    pub language: *const c_char,
    pub confidence: f64,
}

#[repr(C)]
pub struct StringRaw {
    pub value: *mut c_char,
}

#[repr(C)]
pub struct StringDoubleRaw {
    pub key: *mut c_char,
    pub value: f64,
}

#[repr(C)]
pub struct TokenRaw {
    pub start: usize,
    pub length: usize,
    pub text: *mut c_char,
}

#[repr(C)]
pub struct TokenSpanRaw {
    pub start: usize,
    pub length: usize,
    pub text: *mut c_char,
    pub attributes: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TagSpanRaw {
    pub start: usize,
    pub length: usize,
    pub text: *mut c_char,
    pub tag: *mut c_char,
}

#[repr(C)]
pub struct NamedEntityRaw {
    pub start: usize,
    pub length: usize,
    pub text: *mut c_char,
    pub tag: *mut c_char,
}

#[repr(C)]
pub struct BytesRaw {
    pub bytes: *mut c_void,
    pub len: usize,
}

#[repr(C)]
pub struct EmbeddingNeighborRaw {
    pub word: *mut c_char,
    pub distance: f64,
}

#[repr(C)]
pub struct EmbeddingVectorEntryRefRaw {
    pub word: *const c_char,
    pub values: *const f64,
    pub len: usize,
}

#[repr(C)]
pub struct LabelTermRefRaw {
    pub label: *const c_char,
    pub term: *const c_char,
}

#[repr(C)]
pub struct OrthographyEntryRefRaw {
    pub script: *const c_char,
    pub language: *const c_char,
}

#[repr(C)]
pub struct TokenVectorRaw {
    pub start: usize,
    pub length: usize,
    pub values: *mut f64,
    pub len: usize,
}

#[repr(C)]
pub struct HypothesisSetRaw {
    pub entries: *mut c_void,
    pub count: usize,
}

#[repr(C)]
pub struct CoreMlModelRefRaw {
    _private: [u8; 0],
}

extern "C" {
    pub fn nl_string_free(s: *mut c_char);
    pub fn nl_object_retain(handle: *mut c_void) -> *mut c_void;
    pub fn nl_object_release(handle: *mut c_void);

    pub fn nl_strings_free(array: *mut c_void, count: usize);
    pub fn nl_string_doubles_free(array: *mut c_void, count: usize);
    pub fn nl_handle_array_free(array: *mut c_void, count: usize);
    pub fn nl_usizes_free(array: *mut c_void, count: usize);
    pub fn nl_bytes_free(bytes: *mut c_void);
    pub fn nl_doubles_free(values: *mut f64, count: usize);
    pub fn nl_token_spans_free(array: *mut c_void, count: usize);
    pub fn nl_tag_spans_free(array: *mut c_void, count: usize);
    pub fn nl_tag_span_clear(span: *mut c_void);
    pub fn nl_token_vectors_free(array: *mut c_void, count: usize);
    pub fn nl_token_vector_clear(vector: *mut c_void);
    pub fn nl_hypothesis_sets_free(array: *mut c_void, count: usize);

    pub fn nl_dominant_language(text: *const c_char, out_language: *mut *mut c_char) -> i32;
    pub fn nl_language_hypotheses(
        text: *const c_char,
        max_hypotheses: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;
    pub fn nl_language_hypotheses_free(array: *mut c_void, count: usize);

    pub fn nl_language_recognizer_create() -> *mut c_void;
    pub fn nl_language_recognizer_process_string(
        handle: *mut c_void,
        text: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_language_recognizer_reset(handle: *mut c_void);
    pub fn nl_language_recognizer_dominant_language(
        handle: *mut c_void,
        out_language: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_language_recognizer_language_hypotheses(
        handle: *mut c_void,
        max_hypotheses: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_language_recognizer_language_hints(
        handle: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_language_recognizer_set_language_hints(
        handle: *mut c_void,
        hints: *const c_void,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_language_recognizer_language_constraints(
        handle: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_language_recognizer_set_language_constraints(
        handle: *mut c_void,
        constraints: *const *const c_char,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn nl_tokenize(
        text: *const c_char,
        unit: i32,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;
    pub fn nl_tokens_free(array: *mut c_void, count: usize);

    pub fn nl_tokenizer_create(unit: i32) -> *mut c_void;
    pub fn nl_tokenizer_unit(handle: *mut c_void) -> i32;
    pub fn nl_tokenizer_string(
        handle: *mut c_void,
        out_string: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tokenizer_set_string(
        handle: *mut c_void,
        string: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tokenizer_set_language(
        handle: *mut c_void,
        language: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tokenizer_token_range_at_index(
        handle: *mut c_void,
        character_index: usize,
        out_range: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tokenizer_token_range_for_range(
        handle: *mut c_void,
        range_start: usize,
        range_length: usize,
        out_range: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tokenizer_tokens_in_range(
        handle: *mut c_void,
        range_start: usize,
        range_length: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn nl_named_entities(
        text: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;
    pub fn nl_named_entities_free(array: *mut c_void, count: usize);

    pub fn nl_tagger_create(
        schemes: *const *const c_char,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn nl_tagger_tag_schemes(
        handle: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_string(
        handle: *mut c_void,
        out_string: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_set_string(
        handle: *mut c_void,
        string: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_available_tag_schemes(
        unit: i32,
        language: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_token_range_at_index(
        handle: *mut c_void,
        character_index: usize,
        unit: i32,
        out_range: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_token_range_for_range(
        handle: *mut c_void,
        range_start: usize,
        range_length: usize,
        unit: i32,
        out_range: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_dominant_language(
        handle: *mut c_void,
        out_language: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_tags_in_range(
        handle: *mut c_void,
        range_start: usize,
        range_length: usize,
        unit: i32,
        scheme: *const c_char,
        options: u64,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_tag_at_index(
        handle: *mut c_void,
        character_index: usize,
        unit: i32,
        scheme: *const c_char,
        out_span: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_tag_hypotheses_at_index(
        handle: *mut c_void,
        character_index: usize,
        unit: i32,
        scheme: *const c_char,
        maximum_count: usize,
        out_range: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_set_language(
        handle: *mut c_void,
        language: *const c_char,
        range_start: usize,
        range_length: usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_set_orthography(
        handle: *mut c_void,
        dominant_script: *const c_char,
        entries: *const c_void,
        entry_count: usize,
        range_start: usize,
        range_length: usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_set_models(
        handle: *mut c_void,
        models: *const *mut c_void,
        count: usize,
        scheme: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_models_for_tag_scheme(
        handle: *mut c_void,
        scheme: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_set_gazetteers(
        handle: *mut c_void,
        gazetteers: *const *mut c_void,
        count: usize,
        scheme: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_gazetteers_for_tag_scheme(
        handle: *mut c_void,
        scheme: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_tagger_request_assets(
        language: *const c_char,
        scheme: *const c_char,
        out_result: *mut i32,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn nl_word_embedding_for_language(language: *const c_char) -> *mut c_void;
    pub fn nl_sentence_embedding_for_language(language: *const c_char) -> *mut c_void;
    pub fn nl_embedding_release(handle: *mut c_void);
    pub fn nl_embedding_dimension(handle: *mut c_void) -> usize;
    pub fn nl_embedding_vocabulary_size(handle: *mut c_void) -> usize;
    pub fn nl_embedding_vector_for_string(
        handle: *mut c_void,
        word: *const c_char,
        out_buf: *mut f64,
        out_len: usize,
    ) -> bool;
    pub fn nl_embedding_distance(handle: *mut c_void, a: *const c_char, b: *const c_char) -> f64;
    pub fn nl_embedding_neighbors_for_string(
        handle: *mut c_void,
        word: *const c_char,
        max_count: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> bool;
    pub fn nl_embedding_neighbors_free(array: *mut c_void, count: usize);

    pub fn nl_word_embedding_for_language_revision(
        language: *const c_char,
        revision: usize,
    ) -> *mut c_void;
    pub fn nl_sentence_embedding_for_language_revision(
        language: *const c_char,
        revision: usize,
    ) -> *mut c_void;
    pub fn nl_embedding_with_contents_of_url(
        path: *const c_char,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_embedding_contains_string(handle: *mut c_void, word: *const c_char) -> bool;
    pub fn nl_embedding_language(
        handle: *mut c_void,
        out_language: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_embedding_revision(handle: *mut c_void) -> usize;
    pub fn nl_embedding_distance_with_type(
        handle: *mut c_void,
        a: *const c_char,
        b: *const c_char,
        distance_type: i32,
    ) -> f64;
    pub fn nl_embedding_neighbors_for_string_with_limit(
        handle: *mut c_void,
        word: *const c_char,
        max_count: usize,
        max_distance: f64,
        distance_type: i32,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> bool;
    pub fn nl_embedding_neighbors_for_vector(
        handle: *mut c_void,
        values: *const f64,
        len: usize,
        max_count: usize,
        distance_type: i32,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> bool;
    pub fn nl_embedding_neighbors_for_vector_with_limit(
        handle: *mut c_void,
        values: *const f64,
        len: usize,
        max_count: usize,
        max_distance: f64,
        distance_type: i32,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> bool;
    pub fn nl_embedding_supported_revisions_for_language(
        language: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;
    pub fn nl_embedding_current_revision_for_language(language: *const c_char) -> usize;
    pub fn nl_embedding_supported_sentence_revisions_for_language(
        language: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;
    pub fn nl_embedding_current_sentence_revision_for_language(language: *const c_char) -> usize;
    pub fn nl_embedding_write_dictionary(
        entries: *const c_void,
        count: usize,
        language: *const c_char,
        revision: usize,
        path: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn nl_gazetteer_with_contents_of_url(
        path: *const c_char,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_gazetteer_with_data(
        bytes: *const u8,
        len: usize,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_gazetteer_with_dictionary(
        entries: *const c_void,
        count: usize,
        language: *const c_char,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_gazetteer_label_for_string(
        handle: *mut c_void,
        text: *const c_char,
        out_label: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_gazetteer_language(
        handle: *mut c_void,
        out_language: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_gazetteer_data(
        handle: *mut c_void,
        out_bytes: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_gazetteer_write_dictionary(
        entries: *const c_void,
        count: usize,
        language: *const c_char,
        path: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn nl_coreml_model_create_from_source_path(
        path: *const c_char,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_coreml_model_create_from_compiled_path(
        path: *const c_char,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_model_with_contents_of_url(
        path: *const c_char,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_model_with_mlmodel(
        coreml_handle: *mut c_void,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_model_configuration(handle: *mut c_void) -> *mut c_void;
    pub fn nl_model_predicted_label_for_string(
        handle: *mut c_void,
        text: *const c_char,
        out_label: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_model_predicted_labels_for_tokens(
        handle: *mut c_void,
        tokens: *const *const c_char,
        count: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_model_predicted_label_hypotheses_for_string(
        handle: *mut c_void,
        text: *const c_char,
        maximum_count: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_model_predicted_label_hypotheses_for_tokens(
        handle: *mut c_void,
        tokens: *const *const c_char,
        count: usize,
        maximum_count: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_model_configuration_type(handle: *mut c_void) -> i32;
    pub fn nl_model_configuration_language(
        handle: *mut c_void,
        out_language: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_model_configuration_revision(handle: *mut c_void) -> usize;
    pub fn nl_model_supported_revisions_for_type(
        model_type: i32,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;
    pub fn nl_model_current_revision_for_type(model_type: i32) -> usize;

    pub fn nl_contextual_embedding_with_model_identifier(identifier: *const c_char) -> *mut c_void;
    pub fn nl_contextual_embeddings_for_query(
        languages: *const *const c_char,
        language_count: usize,
        scripts: *const *const c_char,
        script_count: usize,
        has_revision: bool,
        revision: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_with_language(language: *const c_char) -> *mut c_void;
    pub fn nl_contextual_embedding_with_script(script: *const c_char) -> *mut c_void;
    pub fn nl_contextual_embedding_model_identifier(
        handle: *mut c_void,
        out_identifier: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_languages(
        handle: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_scripts(
        handle: *mut c_void,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_revision(
        handle: *mut c_void,
        out_revision: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_dimension(
        handle: *mut c_void,
        out_dimension: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_maximum_sequence_length(
        handle: *mut c_void,
        out_value: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_load(handle: *mut c_void, out_error: *mut *mut c_char) -> i32;
    pub fn nl_contextual_embedding_unload(handle: *mut c_void, out_error: *mut *mut c_char) -> i32;
    pub fn nl_contextual_embedding_result_for_string(
        handle: *mut c_void,
        text: *const c_char,
        language: *const c_char,
        out_result: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_has_available_assets(
        handle: *mut c_void,
        out_value: *mut bool,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_request_assets(
        handle: *mut c_void,
        out_result: *mut i32,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_result_string(
        handle: *mut c_void,
        out_string: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_result_language(
        handle: *mut c_void,
        out_language: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_result_sequence_length(handle: *mut c_void) -> usize;
    pub fn nl_contextual_embedding_result_token_vectors_in_range(
        handle: *mut c_void,
        range_start: usize,
        range_length: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn nl_contextual_embedding_result_token_vector_at_index(
        handle: *mut c_void,
        character_index: usize,
        out_vector: *mut c_void,
        out_found: *mut bool,
        out_error: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const NO_DOMINANT_LANGUAGE: i32 = -2;
    pub const UNSUPPORTED: i32 = -3;
    pub const UNKNOWN: i32 = -99;
}
