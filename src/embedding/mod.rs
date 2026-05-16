//! [`Embedding`] — `NLEmbedding` for word / sentence vectors.

use core::ffi::c_void;
use std::ffi::CString;

use crate::error::NLError;
use crate::ffi;

/// A handle to a pre-trained word- or sentence-embedding model.
pub struct Embedding {
    handle: *mut c_void,
}

unsafe impl Send for Embedding {}
unsafe impl Sync for Embedding {}

impl Drop for Embedding {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::nl_embedding_release(self.handle) };
            self.handle = core::ptr::null_mut();
        }
    }
}

impl Embedding {
    /// Load Apple's pre-trained word-embedding model for the given
    /// language (e.g. `"en"`, `"sv"`, `"de"`, `"ja"`).
    ///
    /// # Errors
    ///
    /// Returns `Ok(None)` if Apple has no embedding for that language.
    /// Returns [`NLError::InvalidArgument`] if `language` contains a
    /// NUL byte.
    pub fn word_for_language(language: &str) -> Result<Option<Self>, NLError> {
        let cs = CString::new(language)
            .map_err(|e| NLError::InvalidArgument(format!("language NUL byte: {e}")))?;
        let handle = unsafe { ffi::nl_word_embedding_for_language(cs.as_ptr()) };
        if handle.is_null() {
            Ok(None)
        } else {
            Ok(Some(Self { handle }))
        }
    }

    /// Load Apple's pre-trained sentence-embedding model for the given
    /// language (macOS 11+).
    ///
    /// # Errors
    ///
    /// See [`Self::word_for_language`].
    pub fn sentence_for_language(language: &str) -> Result<Option<Self>, NLError> {
        let cs = CString::new(language)
            .map_err(|e| NLError::InvalidArgument(format!("language NUL byte: {e}")))?;
        let handle = unsafe { ffi::nl_sentence_embedding_for_language(cs.as_ptr()) };
        if handle.is_null() {
            Ok(None)
        } else {
            Ok(Some(Self { handle }))
        }
    }

    /// Dimensionality of the underlying vector space.
    #[must_use]
    pub fn dimension(&self) -> usize {
        unsafe { ffi::nl_embedding_dimension(self.handle) }
    }

    /// Number of in-vocabulary tokens.
    #[must_use]
    pub fn vocabulary_size(&self) -> usize {
        unsafe { ffi::nl_embedding_vocabulary_size(self.handle) }
    }

    /// Return the vector for `word`, or `None` if the word is out of
    /// vocabulary (sentence-embedding models always return `Some`).
    ///
    /// # Errors
    ///
    /// Returns [`NLError::InvalidArgument`] if `word` contains a NUL byte.
    pub fn vector_for(&self, word: &str) -> Result<Option<Vec<f64>>, NLError> {
        let cs = CString::new(word)
            .map_err(|e| NLError::InvalidArgument(format!("word NUL byte: {e}")))?;
        let dim = self.dimension();
        if dim == 0 {
            return Ok(None);
        }
        let mut buf = vec![0.0_f64; dim];
        let ok = unsafe {
            ffi::nl_embedding_vector_for_string(self.handle, cs.as_ptr(), buf.as_mut_ptr(), dim)
        };
        if ok {
            Ok(Some(buf))
        } else {
            Ok(None)
        }
    }

    /// Cosine distance between two in-vocabulary strings. Returns
    /// `None` if either string is out of vocabulary.
    ///
    /// # Errors
    ///
    /// Returns [`NLError::InvalidArgument`] on NUL bytes.
    pub fn distance(&self, a: &str, b: &str) -> Result<Option<f64>, NLError> {
        let ac = CString::new(a)
            .map_err(|e| NLError::InvalidArgument(format!("a NUL byte: {e}")))?;
        let bc = CString::new(b)
            .map_err(|e| NLError::InvalidArgument(format!("b NUL byte: {e}")))?;
        let d = unsafe { ffi::nl_embedding_distance(self.handle, ac.as_ptr(), bc.as_ptr()) };
        if d < 0.0 {
            Ok(None)
        } else {
            Ok(Some(d))
        }
    }

    /// Return up to `max` nearest neighbours of `word` with their
    /// cosine distances.
    ///
    /// # Errors
    ///
    /// Returns [`NLError::InvalidArgument`] on NUL byte.
    pub fn neighbors(&self, word: &str, max: usize) -> Result<Vec<Neighbor>, NLError> {
        let cs = CString::new(word)
            .map_err(|e| NLError::InvalidArgument(format!("word NUL byte: {e}")))?;
        let mut out_array: *mut c_void = core::ptr::null_mut();
        let mut out_count: usize = 0;
        let ok = unsafe {
            ffi::nl_embedding_neighbors_for_string(
                self.handle,
                cs.as_ptr(),
                max,
                &mut out_array,
                &mut out_count,
            )
        };
        if !ok || out_array.is_null() || out_count == 0 {
            return Ok(Vec::new());
        }
        let typed = out_array.cast::<ffi::EmbeddingNeighborRaw>();
        let mut v = Vec::with_capacity(out_count);
        for i in 0..out_count {
            let raw = unsafe { &*typed.add(i) };
            let word = if raw.word.is_null() {
                String::new()
            } else {
                unsafe { core::ffi::CStr::from_ptr(raw.word) }
                    .to_string_lossy()
                    .into_owned()
            };
            v.push(Neighbor {
                word,
                distance: raw.distance,
            });
        }
        unsafe { ffi::nl_embedding_neighbors_free(out_array, out_count) };
        Ok(v)
    }
}

/// One nearest-neighbour result from
/// [`Embedding::neighbors`].
#[derive(Debug, Clone, PartialEq)]
pub struct Neighbor {
    pub word: String,
    /// Cosine distance — smaller is closer.
    pub distance: f64,
}
