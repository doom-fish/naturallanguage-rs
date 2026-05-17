//! [`Embedding`] — `NLEmbedding` for word / sentence vectors.

use core::ffi::{c_char, c_void};
use std::ptr::{self, NonNull};

use crate::error::NLError;
use crate::ffi;
use crate::language::Language;
use crate::util::{cstring_arg, decode_usize_array, status_error, take_string};

/// `NLDistanceType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DistanceType {
    Cosine = 0,
}

/// `NLDistance`.
pub type Distance = f64;

/// One dictionary entry accepted by `Embedding::write_dictionary`.
#[derive(Debug, Clone, Copy)]
pub struct EmbeddingDictionaryEntry<'a> {
    pub token: &'a str,
    pub values: &'a [f64],
}

/// A handle to a pre-trained or file-backed embedding model.
#[derive(Debug)]
pub struct Embedding {
    handle: NonNull<c_void>,
}

unsafe impl Send for Embedding {}
unsafe impl Sync for Embedding {}

impl Drop for Embedding {
    fn drop(&mut self) {
        unsafe { ffi::nl_embedding_release(self.handle.as_ptr()) };
    }
}

impl Embedding {
    pub fn word_for_language(language: impl AsRef<str>) -> Result<Option<Self>, NLError> {
        let cs = cstring_arg(language.as_ref(), "language")?;
        Ok(
            NonNull::new(unsafe { ffi::nl_word_embedding_for_language(cs.as_ptr()) })
                .map(|handle| Self { handle }),
        )
    }

    pub fn word_for_language_revision(
        language: impl AsRef<str>,
        revision: usize,
    ) -> Result<Option<Self>, NLError> {
        let cs = cstring_arg(language.as_ref(), "language")?;
        Ok(NonNull::new(unsafe {
            ffi::nl_word_embedding_for_language_revision(cs.as_ptr(), revision)
        })
        .map(|handle| Self { handle }))
    }

    pub fn sentence_for_language(language: impl AsRef<str>) -> Result<Option<Self>, NLError> {
        let cs = cstring_arg(language.as_ref(), "language")?;
        Ok(
            NonNull::new(unsafe { ffi::nl_sentence_embedding_for_language(cs.as_ptr()) })
                .map(|handle| Self { handle }),
        )
    }

    pub fn sentence_for_language_revision(
        language: impl AsRef<str>,
        revision: usize,
    ) -> Result<Option<Self>, NLError> {
        let cs = cstring_arg(language.as_ref(), "language")?;
        Ok(NonNull::new(unsafe {
            ffi::nl_sentence_embedding_for_language_revision(cs.as_ptr(), revision)
        })
        .map(|handle| Self { handle }))
    }

    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, NLError> {
        let path_c = cstring_arg(&path.as_ref().to_string_lossy(), "path")?;
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_embedding_with_contents_of_url(path_c.as_ptr(), &mut handle, &mut error)
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "failed to load embedding", error));
        }
        let handle = NonNull::new(handle).ok_or_else(|| NLError::Unknown {
            code: ffi::status::UNKNOWN,
            message: "NaturalLanguage returned a null embedding".into(),
        })?;
        Ok(Self { handle })
    }

    #[must_use]
    pub fn dimension(&self) -> usize {
        unsafe { ffi::nl_embedding_dimension(self.handle.as_ptr()) }
    }

    #[must_use]
    pub fn vocabulary_size(&self) -> usize {
        unsafe { ffi::nl_embedding_vocabulary_size(self.handle.as_ptr()) }
    }

    pub fn language(&self) -> Result<Option<Language>, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::nl_embedding_language(self.handle.as_ptr(), &mut out, &mut error) };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) }.map(Language::from))
        } else {
            Err(status_error(
                status,
                "embedding language query failed",
                error,
            ))
        }
    }

    #[must_use]
    pub fn revision(&self) -> usize {
        unsafe { ffi::nl_embedding_revision(self.handle.as_ptr()) }
    }

    pub fn contains_string(&self, word: &str) -> Result<bool, NLError> {
        let cs = cstring_arg(word, "word")?;
        Ok(unsafe { ffi::nl_embedding_contains_string(self.handle.as_ptr(), cs.as_ptr()) })
    }

    pub fn vector_for(&self, word: &str) -> Result<Option<Vec<f64>>, NLError> {
        let cs = cstring_arg(word, "word")?;
        let dim = self.dimension();
        if dim == 0 {
            return Ok(None);
        }
        let mut buf = vec![0.0_f64; dim];
        let ok = unsafe {
            ffi::nl_embedding_vector_for_string(
                self.handle.as_ptr(),
                cs.as_ptr(),
                buf.as_mut_ptr(),
                dim,
            )
        };
        if ok {
            Ok(Some(buf))
        } else {
            Ok(None)
        }
    }

    pub fn distance(&self, a: &str, b: &str) -> Result<Option<Distance>, NLError> {
        self.distance_with_type(a, b, DistanceType::Cosine)
    }

    pub fn distance_with_type(
        &self,
        a: &str,
        b: &str,
        distance_type: DistanceType,
    ) -> Result<Option<Distance>, NLError> {
        let ac = cstring_arg(a, "a")?;
        let bc = cstring_arg(b, "b")?;
        let distance = unsafe {
            ffi::nl_embedding_distance_with_type(
                self.handle.as_ptr(),
                ac.as_ptr(),
                bc.as_ptr(),
                distance_type as i32,
            )
        };
        if distance < 0.0 {
            Ok(None)
        } else {
            Ok(Some(distance))
        }
    }

    pub fn neighbors(&self, word: &str, max: usize) -> Result<Vec<Neighbor>, NLError> {
        self.neighbors_with_limit(word, max, None, DistanceType::Cosine)
    }

    pub fn neighbors_with_limit(
        &self,
        word: &str,
        max: usize,
        maximum_distance: Option<Distance>,
        distance_type: DistanceType,
    ) -> Result<Vec<Neighbor>, NLError> {
        let cs = cstring_arg(word, "word")?;
        let mut out_array: *mut c_void = ptr::null_mut();
        let mut out_count: usize = 0;
        let ok = unsafe {
            ffi::nl_embedding_neighbors_for_string_with_limit(
                self.handle.as_ptr(),
                cs.as_ptr(),
                max,
                maximum_distance.unwrap_or(-1.0),
                distance_type as i32,
                &mut out_array,
                &mut out_count,
            )
        };
        if !ok {
            return Ok(Vec::new());
        }
        Ok(unsafe { decode_neighbors(out_array, out_count) })
    }

    pub fn neighbors_for_vector(
        &self,
        vector: &[f64],
        max: usize,
        distance_type: DistanceType,
    ) -> Result<Vec<Neighbor>, NLError> {
        self.neighbors_for_vector_with_limit(vector, max, None, distance_type)
    }

    pub fn neighbors_for_vector_with_limit(
        &self,
        vector: &[f64],
        max: usize,
        maximum_distance: Option<Distance>,
        distance_type: DistanceType,
    ) -> Result<Vec<Neighbor>, NLError> {
        let mut out_array: *mut c_void = ptr::null_mut();
        let mut out_count: usize = 0;
        let ok = unsafe {
            ffi::nl_embedding_neighbors_for_vector_with_limit(
                self.handle.as_ptr(),
                vector.as_ptr(),
                vector.len(),
                max,
                maximum_distance.unwrap_or(-1.0),
                distance_type as i32,
                &mut out_array,
                &mut out_count,
            )
        };
        if !ok {
            return Ok(Vec::new());
        }
        Ok(unsafe { decode_neighbors(out_array, out_count) })
    }

    pub fn enumerate_neighbors_for_string<F>(
        &self,
        word: &str,
        max: usize,
        maximum_distance: Option<Distance>,
        distance_type: DistanceType,
        mut callback: F,
    ) -> Result<(), NLError>
    where
        F: FnMut(&Neighbor) -> bool,
    {
        for neighbor in self.neighbors_with_limit(word, max, maximum_distance, distance_type)? {
            if !callback(&neighbor) {
                break;
            }
        }
        Ok(())
    }

    pub fn enumerate_neighbors_for_vector<F>(
        &self,
        vector: &[f64],
        max: usize,
        maximum_distance: Option<Distance>,
        distance_type: DistanceType,
        mut callback: F,
    ) -> Result<(), NLError>
    where
        F: FnMut(&Neighbor) -> bool,
    {
        for neighbor in
            self.neighbors_for_vector_with_limit(vector, max, maximum_distance, distance_type)?
        {
            if !callback(&neighbor) {
                break;
            }
        }
        Ok(())
    }

    pub fn supported_revisions_for_language(language: &Language) -> Result<Vec<usize>, NLError> {
        let cs = cstring_arg(language.as_str(), "language")?;
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let status = unsafe {
            ffi::nl_embedding_supported_revisions_for_language(cs.as_ptr(), &mut array, &mut count)
        };
        if status == ffi::status::OK {
            Ok(unsafe { decode_usize_array(array, count) })
        } else {
            Err(NLError::Unknown {
                code: status,
                message: "supported embedding revisions query failed".into(),
            })
        }
    }

    #[must_use]
    pub fn current_revision_for_language(language: &Language) -> usize {
        let cs = std::ffi::CString::new(language.as_str())
            .expect("language constant must not contain NUL");
        unsafe { ffi::nl_embedding_current_revision_for_language(cs.as_ptr()) }
    }

    pub fn supported_sentence_revisions_for_language(
        language: &Language,
    ) -> Result<Vec<usize>, NLError> {
        let cs = cstring_arg(language.as_str(), "language")?;
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let status = unsafe {
            ffi::nl_embedding_supported_sentence_revisions_for_language(
                cs.as_ptr(),
                &mut array,
                &mut count,
            )
        };
        if status == ffi::status::OK {
            Ok(unsafe { decode_usize_array(array, count) })
        } else {
            Err(NLError::Unknown {
                code: status,
                message: "supported sentence embedding revisions query failed".into(),
            })
        }
    }

    #[must_use]
    pub fn current_sentence_revision_for_language(language: &Language) -> usize {
        let cs = std::ffi::CString::new(language.as_str())
            .expect("language constant must not contain NUL");
        unsafe { ffi::nl_embedding_current_sentence_revision_for_language(cs.as_ptr()) }
    }

    pub fn write_dictionary(
        entries: &[EmbeddingDictionaryEntry<'_>],
        language: Option<&Language>,
        revision: usize,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), NLError> {
        let words = entries
            .iter()
            .map(|entry| cstring_arg(entry.token, "token"))
            .collect::<Result<Vec<_>, _>>()?;
        let refs = entries
            .iter()
            .zip(&words)
            .map(|(entry, token)| ffi::EmbeddingVectorEntryRefRaw {
                word: token.as_ptr(),
                values: entry.values.as_ptr(),
                len: entry.values.len(),
            })
            .collect::<Vec<_>>();
        let language_c = language
            .map(|value| cstring_arg(value.as_str(), "language"))
            .transpose()?;
        let path_c = cstring_arg(&path.as_ref().to_string_lossy(), "path")?;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_embedding_write_dictionary(
                refs.as_ptr().cast(),
                refs.len(),
                language_c
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                revision,
                path_c.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(
                status,
                "writing embedding dictionary failed",
                error,
            ))
        }
    }
}

/// One nearest-neighbour result from [`Embedding::neighbors`].
#[derive(Debug, Clone, PartialEq)]
pub struct Neighbor {
    pub word: String,
    /// Cosine distance — smaller is closer.
    pub distance: Distance,
}

unsafe fn decode_neighbors(array: *mut c_void, count: usize) -> Vec<Neighbor> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::EmbeddingNeighborRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        let raw = &*typed.add(idx);
        let word = if raw.word.is_null() {
            String::new()
        } else {
            core::ffi::CStr::from_ptr(raw.word)
                .to_string_lossy()
                .into_owned()
        };
        values.push(Neighbor {
            word,
            distance: raw.distance,
        });
    }
    ffi::nl_embedding_neighbors_free(array, count);
    values
}
