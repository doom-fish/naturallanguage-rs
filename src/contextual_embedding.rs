//! `NLContextualEmbedding` bindings (macOS 14+).

use core::ffi::{c_char, c_void};
use std::ptr::{self, NonNull};

use crate::error::NLError;
use crate::ffi;
use crate::language::Language;
use crate::script::Script;
use crate::types::TextRange;
use crate::util::{cstring_arg, decode_string_array, status_error, take_string};

/// Query used by `NLContextualEmbedding.contextualEmbeddingsForValues:`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContextualEmbeddingQuery {
    pub languages: Vec<Language>,
    pub scripts: Vec<Script>,
    pub revision: Option<usize>,
}

/// Result of requesting contextual embedding assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ContextualEmbeddingAssetsResult {
    Available = 0,
    NotAvailable = 1,
    Error = 2,
}

/// One contextual token vector.
#[derive(Debug, Clone, PartialEq)]
pub struct TokenVector {
    pub range: TextRange,
    pub values: Vec<f64>,
}

/// Result object returned from applying a contextual embedding.
#[derive(Debug)]
pub struct ContextualEmbeddingResult {
    handle: NonNull<c_void>,
}

// SAFETY: ContextualEmbeddingResult wraps an Objective-C object handle from NaturalLanguage.framework,
// which is thread-safe. Rust holds exclusive ownership of the handle, and the framework's
// internal locking ensures thread-safe access.
unsafe impl Send for ContextualEmbeddingResult {}

// SAFETY: The underlying NLContextualEmbeddingResult object is thread-safe.
unsafe impl Sync for ContextualEmbeddingResult {}

impl Drop for ContextualEmbeddingResult {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl ContextualEmbeddingResult {
    pub fn string(&self) -> Result<String, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_result_string(self.handle.as_ptr(), &mut out, &mut error)
        };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) }.unwrap_or_default())
        } else {
            Err(status_error(
                status,
                "contextual embedding result string failed",
                error,
            ))
        }
    }

    pub fn language(&self) -> Result<Language, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_result_language(self.handle.as_ptr(), &mut out, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Language::from(
                unsafe { take_string(out) }.unwrap_or_default(),
            ))
        } else {
            Err(status_error(
                status,
                "contextual embedding result language failed",
                error,
            ))
        }
    }

    #[must_use]
    pub fn sequence_length(&self) -> usize {
        unsafe { ffi::nl_contextual_embedding_result_sequence_length(self.handle.as_ptr()) }
    }

    pub fn token_vectors_in_range(&self, range: TextRange) -> Result<Vec<TokenVector>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_result_token_vectors_in_range(
                self.handle.as_ptr(),
                range.start,
                range.length,
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "token_vectors_in_range failed", error));
        }
        Ok(unsafe { decode_token_vectors(array, count) })
    }

    pub fn enumerate_token_vectors_in_range<F>(
        &self,
        range: TextRange,
        mut callback: F,
    ) -> Result<(), NLError>
    where
        F: FnMut(&TokenVector) -> bool,
    {
        for vector in self.token_vectors_in_range(range)? {
            if !callback(&vector) {
                break;
            }
        }
        Ok(())
    }

    pub fn token_vector_at_index(
        &self,
        character_index: usize,
    ) -> Result<Option<TokenVector>, NLError> {
        let mut raw = ffi::TokenVectorRaw {
            start: 0,
            length: 0,
            values: ptr::null_mut(),
            len: 0,
        };
        let mut found = false;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_result_token_vector_at_index(
                self.handle.as_ptr(),
                character_index,
                ptr::addr_of_mut!(raw).cast(),
                &mut found,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "token_vector_at_index failed", error));
        }
        if !found {
            return Ok(None);
        }
        let values = unsafe {
            if raw.values.is_null() || raw.len == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(raw.values, raw.len).to_vec()
            }
        };
        unsafe { ffi::nl_token_vector_clear(ptr::addr_of_mut!(raw).cast()) };
        Ok(Some(TokenVector {
            range: TextRange::new(raw.start, raw.length),
            values,
        }))
    }

    pub(crate) const unsafe fn from_retained_ptr(handle: NonNull<c_void>) -> Self {
        Self { handle }
    }
}

/// A contextual embedding catalog/model handle.
#[derive(Debug)]
pub struct ContextualEmbedding {
    handle: NonNull<c_void>,
}

// SAFETY: ContextualEmbedding wraps an Objective-C object handle from NaturalLanguage.framework,
// which is thread-safe. Rust holds exclusive ownership of the handle, and the framework's
// internal locking ensures thread-safe access.
unsafe impl Send for ContextualEmbedding {}

// SAFETY: The underlying NLContextualEmbedding object is thread-safe.
unsafe impl Sync for ContextualEmbedding {}

impl Drop for ContextualEmbedding {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl ContextualEmbedding {
    pub fn from_model_identifier(identifier: &str) -> Result<Option<Self>, NLError> {
        let identifier_c = cstring_arg(identifier, "model identifier")?;
        Ok(NonNull::new(unsafe {
            ffi::nl_contextual_embedding_with_model_identifier(identifier_c.as_ptr())
        })
        .map(|handle| Self { handle }))
    }

    pub fn catalog(query: &ContextualEmbeddingQuery) -> Result<Vec<Self>, NLError> {
        let languages = query
            .languages
            .iter()
            .map(|language| cstring_arg(language.as_str(), "language"))
            .collect::<Result<Vec<_>, _>>()?;
        let scripts = query
            .scripts
            .iter()
            .map(|script| cstring_arg(script.as_str(), "script"))
            .collect::<Result<Vec<_>, _>>()?;
        let language_ptrs = languages
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        let script_ptrs = scripts
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embeddings_for_query(
                language_ptrs.as_ptr(),
                language_ptrs.len(),
                script_ptrs.as_ptr(),
                script_ptrs.len(),
                query.revision.is_some(),
                query.revision.unwrap_or_default(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "contextual embedding catalog query failed",
                error,
            ));
        }
        Ok(unsafe { decode_handles(array, count) })
    }

    pub fn for_language(language: &Language) -> Result<Option<Self>, NLError> {
        let language_c = cstring_arg(language.as_str(), "language")?;
        Ok(
            NonNull::new(unsafe {
                ffi::nl_contextual_embedding_with_language(language_c.as_ptr())
            })
            .map(|handle| Self { handle }),
        )
    }

    pub fn for_script(script: &Script) -> Result<Option<Self>, NLError> {
        let script_c = cstring_arg(script.as_str(), "script")?;
        Ok(
            NonNull::new(unsafe { ffi::nl_contextual_embedding_with_script(script_c.as_ptr()) })
                .map(|handle| Self { handle }),
        )
    }

    pub fn model_identifier(&self) -> Result<String, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_model_identifier(
                self.handle.as_ptr(),
                &mut out,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) }.unwrap_or_default())
        } else {
            Err(status_error(
                status,
                "contextual embedding model_identifier failed",
                error,
            ))
        }
    }

    pub fn languages(&self) -> Result<Vec<Language>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_languages(
                self.handle.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "contextual embedding languages failed",
                error,
            ));
        }
        Ok(unsafe { decode_string_array(array, count) }
            .into_iter()
            .map(Language::from)
            .collect())
    }

    pub fn scripts(&self) -> Result<Vec<Script>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_scripts(
                self.handle.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "contextual embedding scripts failed",
                error,
            ));
        }
        Ok(unsafe { decode_string_array(array, count) }
            .into_iter()
            .map(Script::from)
            .collect())
    }

    pub fn revision(&self) -> Result<usize, NLError> {
        let mut out = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_revision(self.handle.as_ptr(), &mut out, &mut error)
        };
        if status == ffi::status::OK {
            Ok(out)
        } else {
            Err(status_error(
                status,
                "contextual embedding revision failed",
                error,
            ))
        }
    }

    pub fn dimension(&self) -> Result<usize, NLError> {
        let mut out = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_dimension(self.handle.as_ptr(), &mut out, &mut error)
        };
        if status == ffi::status::OK {
            Ok(out)
        } else {
            Err(status_error(
                status,
                "contextual embedding dimension failed",
                error,
            ))
        }
    }

    pub fn maximum_sequence_length(&self) -> Result<usize, NLError> {
        let mut out = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_maximum_sequence_length(
                self.handle.as_ptr(),
                &mut out,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(out)
        } else {
            Err(status_error(
                status,
                "contextual embedding maximum_sequence_length failed",
                error,
            ))
        }
    }

    pub fn load(&self) -> Result<(), NLError> {
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::nl_contextual_embedding_load(self.handle.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(
                status,
                "contextual embedding load failed",
                error,
            ))
        }
    }

    pub fn unload(&self) -> Result<(), NLError> {
        let mut error: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::nl_contextual_embedding_unload(self.handle.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(
                status,
                "contextual embedding unload failed",
                error,
            ))
        }
    }

    pub fn embedding_result_for_string(
        &self,
        text: &str,
        language: Option<&Language>,
    ) -> Result<Option<ContextualEmbeddingResult>, NLError> {
        let text_c = cstring_arg(text, "text")?;
        let language_c = language
            .map(|value| cstring_arg(value.as_str(), "language"))
            .transpose()?;
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_result_for_string(
                self.handle.as_ptr(),
                text_c.as_ptr(),
                language_c
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut handle,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "contextual embedding inference failed",
                error,
            ));
        }
        Ok(NonNull::new(handle)
            .map(|handle| unsafe { ContextualEmbeddingResult::from_retained_ptr(handle) }))
    }

    pub fn has_available_assets(&self) -> Result<bool, NLError> {
        let mut out = false;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_has_available_assets(
                self.handle.as_ptr(),
                &mut out,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(out)
        } else {
            Err(status_error(
                status,
                "contextual embedding asset availability failed",
                error,
            ))
        }
    }

    pub fn request_embedding_assets(&self) -> Result<ContextualEmbeddingAssetsResult, NLError> {
        let mut result = ContextualEmbeddingAssetsResult::Error as i32;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_contextual_embedding_request_assets(
                self.handle.as_ptr(),
                &mut result,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(match result {
                0 => ContextualEmbeddingAssetsResult::Available,
                1 => ContextualEmbeddingAssetsResult::NotAvailable,
                _ => ContextualEmbeddingAssetsResult::Error,
            })
        } else {
            Err(status_error(
                status,
                "contextual embedding asset request failed",
                error,
            ))
        }
    }
}

unsafe fn decode_handles(array: *mut c_void, count: usize) -> Vec<ContextualEmbedding> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<*mut c_void>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        if let Some(handle) = NonNull::new(*typed.add(idx)) {
            values.push(ContextualEmbedding { handle });
        }
    }
    ffi::nl_handle_array_free(array, count);
    values
}

unsafe fn decode_token_vectors(array: *mut c_void, count: usize) -> Vec<TokenVector> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::TokenVectorRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        let raw = &*typed.add(idx);
        let vector = if raw.values.is_null() || raw.len == 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(raw.values, raw.len).to_vec()
        };
        values.push(TokenVector {
            range: TextRange::new(raw.start, raw.length),
            values: vector,
        });
    }
    ffi::nl_token_vectors_free(array, count);
    values
}
