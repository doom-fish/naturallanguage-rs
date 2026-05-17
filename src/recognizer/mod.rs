//! Language detection — wraps `NLLanguageRecognizer`.

use core::ffi::{c_char, c_void};
use std::ptr::{self, NonNull};

use crate::error::NLError;
use crate::ffi;
use crate::language::Language;
use crate::util::{cstring_arg, decode_string_array, status_error, take_string};

/// One ranked language hypothesis returned by [`language_hypotheses`] or used
/// as a recognizer hint.
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageHypothesis {
    /// BCP-47 language identifier (e.g. `"en"`, `"sv"`, `"zh-Hans"`).
    pub language: String,
    /// Confidence score in `0.0..=1.0`. Higher is more confident.
    pub confidence: f64,
}

/// Stateful wrapper over Apple's `NLLanguageRecognizer`.
#[derive(Debug)]
pub struct LanguageRecognizer {
    handle: NonNull<c_void>,
}

// SAFETY: LanguageRecognizer wraps an Objective-C object handle from NaturalLanguage.framework,
// which is thread-safe. Rust holds exclusive ownership of the handle, and the framework's
// internal locking ensures thread-safe access.
unsafe impl Send for LanguageRecognizer {}

// SAFETY: The underlying NLLanguageRecognizer object is thread-safe.
unsafe impl Sync for LanguageRecognizer {}

impl Drop for LanguageRecognizer {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl LanguageRecognizer {
    /// Create a new empty recognizer.
    pub fn new() -> Result<Self, NLError> {
        let handle =
            NonNull::new(unsafe { ffi::nl_language_recognizer_create() }).ok_or_else(|| {
                NLError::Unknown {
                    code: ffi::status::UNKNOWN,
                    message: "failed to create language recognizer".into(),
                }
            })?;
        Ok(Self { handle })
    }

    /// Feed a chunk of text into the recognizer.
    pub fn process(&mut self, text: &str) -> Result<(), NLError> {
        let text_c = cstring_arg(text, "text")?;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_language_recognizer_process_string(
                self.handle.as_ptr(),
                text_c.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(
                status,
                "language recognizer process failed",
                error,
            ))
        }
    }

    /// Reset accumulated state so the recognizer can be reused.
    pub fn reset(&mut self) {
        unsafe { ffi::nl_language_recognizer_reset(self.handle.as_ptr()) };
    }

    /// The recognizer's current dominant language.
    pub fn dominant_language(&self) -> Result<Option<Language>, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_language_recognizer_dominant_language(
                self.handle.as_ptr(),
                &mut out,
                &mut error,
            )
        };
        match status {
            ffi::status::OK => Ok(unsafe { take_string(out) }.map(Language::from)),
            ffi::status::NO_DOMINANT_LANGUAGE => Ok(None),
            _ => Err(status_error(
                status,
                "language recognizer dominant language failed",
                error,
            )),
        }
    }

    /// Ranked language candidates for the accumulated text.
    pub fn language_hypotheses(
        &self,
        max_hypotheses: usize,
    ) -> Result<Vec<LanguageHypothesis>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_language_recognizer_language_hypotheses(
                self.handle.as_ptr(),
                max_hypotheses,
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "language recognizer hypotheses failed",
                error,
            ));
        }
        Ok(unsafe { decode_hypotheses(array, count) })
    }

    /// Current prior probabilities used as recognizer hints.
    pub fn language_hints(&self) -> Result<Vec<LanguageHypothesis>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_language_recognizer_language_hints(
                self.handle.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "language recognizer hints failed",
                error,
            ));
        }
        Ok(unsafe { decode_hypotheses(array, count) })
    }

    /// Replace the recognizer's language hints.
    pub fn set_language_hints(&mut self, hints: &[LanguageHypothesis]) -> Result<(), NLError> {
        let languages = hints
            .iter()
            .map(|hint| cstring_arg(&hint.language, "language"))
            .collect::<Result<Vec<_>, _>>()?;
        let raws = hints
            .iter()
            .zip(&languages)
            .map(|(hint, language)| ffi::LanguageHypothesisRefRaw {
                language: language.as_ptr(),
                confidence: hint.confidence,
            })
            .collect::<Vec<_>>();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_language_recognizer_set_language_hints(
                self.handle.as_ptr(),
                raws.as_ptr().cast(),
                raws.len(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(status, "setting language hints failed", error))
        }
    }

    /// Languages the recognizer is constrained to consider.
    pub fn language_constraints(&self) -> Result<Vec<Language>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_language_recognizer_language_constraints(
                self.handle.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "language recognizer constraints failed",
                error,
            ));
        }
        Ok(unsafe { decode_string_array(array, count) }
            .into_iter()
            .map(Language::from)
            .collect())
    }

    /// Replace the recognizer's language constraints.
    pub fn set_language_constraints(&mut self, constraints: &[Language]) -> Result<(), NLError> {
        let raw_languages = constraints
            .iter()
            .map(|language| cstring_arg(language.as_str(), "language"))
            .collect::<Result<Vec<_>, _>>()?;
        let raw_ptrs = raw_languages
            .iter()
            .map(|language| language.as_ptr())
            .collect::<Vec<_>>();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_language_recognizer_set_language_constraints(
                self.handle.as_ptr(),
                raw_ptrs.as_ptr(),
                raw_ptrs.len(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(
                status,
                "setting language constraints failed",
                error,
            ))
        }
    }
}

/// Detect the dominant language of `text`, returning a BCP-47 identifier
/// (e.g. `"en"`, `"sv"`, `"ja"`), or `None` if the recognizer can't decide.
///
/// # Errors
///
/// Returns [`NLError::InvalidArgument`] if `text` contains an interior
/// NUL byte.
///
/// # Examples
///
/// ```rust,no_run
/// use naturallanguage::recognizer::dominant_language;
///
/// let lang = dominant_language("The quick brown fox").unwrap();
/// assert_eq!(lang, Some("en".to_string()));
/// ```
pub fn dominant_language(text: &str) -> Result<Option<String>, NLError> {
    let mut recognizer = LanguageRecognizer::new()?;
    recognizer.process(text)?;
    recognizer
        .dominant_language()
        .map(|language| language.map(|value| value.as_str().to_string()))
}

/// Return up to `max_hypotheses` ranked (language, confidence) pairs for
/// `text`, sorted by descending confidence.
///
/// # Errors
///
/// Returns [`NLError::InvalidArgument`] for invalid input strings.
pub fn language_hypotheses(
    text: &str,
    max_hypotheses: usize,
) -> Result<Vec<LanguageHypothesis>, NLError> {
    let mut recognizer = LanguageRecognizer::new()?;
    recognizer.process(text)?;
    recognizer.language_hypotheses(max_hypotheses)
}

unsafe fn decode_hypotheses(array: *mut c_void, count: usize) -> Vec<LanguageHypothesis> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::LanguageHypothesisRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        let raw = &*typed.add(idx);
        let language = if raw.language.is_null() {
            String::new()
        } else {
            core::ffi::CStr::from_ptr(raw.language)
                .to_string_lossy()
                .into_owned()
        };
        values.push(LanguageHypothesis {
            language,
            confidence: raw.confidence,
        });
    }
    ffi::nl_language_hypotheses_free(array, count);
    values
}
