//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct LanguageHypothesisRaw {
    pub language: *mut c_char,
    pub confidence: f64,
}

#[repr(C)]
pub struct TokenRaw {
    pub start: usize,
    pub length: usize,
    pub text: *mut c_char,
}

#[repr(C)]
pub struct NamedEntityRaw {
    pub start: usize,
    pub length: usize,
    pub text: *mut c_char,
    pub tag: *mut c_char,
}

extern "C" {
    pub fn nl_string_free(s: *mut c_char);

    pub fn nl_dominant_language(
        text: *const c_char,
        out_language: *mut *mut c_char,
    ) -> i32;

    pub fn nl_language_hypotheses(
        text: *const c_char,
        max_hypotheses: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;

    pub fn nl_language_hypotheses_free(array: *mut c_void, count: usize);

    pub fn nl_tokenize(
        text: *const c_char,
        unit: i32,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;

    pub fn nl_tokens_free(array: *mut c_void, count: usize);

    pub fn nl_named_entities(
        text: *const c_char,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;

    pub fn nl_named_entities_free(array: *mut c_void, count: usize);

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
    pub fn nl_embedding_distance(
        handle: *mut c_void,
        a: *const c_char,
        b: *const c_char,
    ) -> f64;
    pub fn nl_embedding_neighbors_for_string(
        handle: *mut c_void,
        word: *const c_char,
        max_count: usize,
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> bool;
    pub fn nl_embedding_neighbors_free(array: *mut c_void, count: usize);
}

#[repr(C)]
pub struct EmbeddingNeighborRaw {
    pub word: *mut c_char,
    pub distance: f64,
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const NO_DOMINANT_LANGUAGE: i32 = -2;
    pub const UNKNOWN: i32 = -99;
}
