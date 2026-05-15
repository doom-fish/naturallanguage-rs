//! Language detection — wraps `NLLanguageRecognizer`.

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;

use crate::error::NLError;
use crate::ffi;

/// One ranked language hypothesis returned by [`language_hypotheses`].
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageHypothesis {
    /// BCP-47 language identifier (e.g. `"en"`, `"sv"`, `"zh-Hans"`).
    pub language: String,
    /// Confidence score in `0.0..=1.0`. Higher is more confident.
    pub confidence: f64,
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
    let text_c =
        CString::new(text).map_err(|e| NLError::InvalidArgument(format!("text NUL byte: {e}")))?;
    let mut out: *mut c_char = ptr::null_mut();
    let status = unsafe { ffi::nl_dominant_language(text_c.as_ptr(), &mut out) };
    match status {
        ffi::status::OK => {
            if out.is_null() {
                return Ok(None);
            }
            let s = unsafe { core::ffi::CStr::from_ptr(out) }
                .to_string_lossy()
                .into_owned();
            unsafe { ffi::nl_string_free(out) };
            Ok(Some(s))
        }
        ffi::status::NO_DOMINANT_LANGUAGE => Ok(None),
        code => Err(NLError::Unknown {
            code,
            message: "dominant language detection failed".into(),
        }),
    }
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
    let text_c =
        CString::new(text).map_err(|e| NLError::InvalidArgument(format!("text NUL byte: {e}")))?;
    let mut array: *mut c_void = ptr::null_mut();
    let mut count: usize = 0;
    let status = unsafe {
        ffi::nl_language_hypotheses(text_c.as_ptr(), max_hypotheses, &mut array, &mut count)
    };
    if status != ffi::status::OK {
        return Err(NLError::Unknown {
            code: status,
            message: "language hypotheses failed".into(),
        });
    }
    if array.is_null() || count == 0 {
        return Ok(Vec::new());
    }
    let typed = array.cast::<ffi::LanguageHypothesisRaw>();
    let mut v = Vec::with_capacity(count);
    for i in 0..count {
        let raw = unsafe { &*typed.add(i) };
        let language = if raw.language.is_null() {
            String::new()
        } else {
            unsafe { core::ffi::CStr::from_ptr(raw.language) }
                .to_string_lossy()
                .into_owned()
        };
        v.push(LanguageHypothesis {
            language,
            confidence: raw.confidence,
        });
    }
    unsafe { ffi::nl_language_hypotheses_free(array, count) };
    Ok(v)
}
