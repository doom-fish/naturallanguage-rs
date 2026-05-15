//! Tokenisation — wraps `NLTokenizer`.

use core::ffi::c_void;
use core::ptr;
use std::ffi::CString;

use crate::error::NLError;
use crate::ffi;

/// Granularity of tokenisation. Maps directly onto Apple's `NLTokenUnit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TokenUnit {
    Word = 0,
    Sentence = 1,
    Paragraph = 2,
    Document = 3,
}

/// One token, with byte-offset position in the source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// UTF-16 character offset of the token start in the source.
    pub start: usize,
    /// UTF-16 character length of the token.
    pub length: usize,
    /// The token text itself.
    pub text: String,
}

/// Tokenise `text` into [`TokenUnit`]-sized chunks.
///
/// # Errors
///
/// Returns [`NLError::InvalidArgument`] if `text` contains an interior NUL.
///
/// # Examples
///
/// ```rust,no_run
/// use naturallanguage::tokenizer::{tokenize, TokenUnit};
///
/// let tokens = tokenize("Hello, world!", TokenUnit::Word).unwrap();
/// assert_eq!(tokens.iter().map(|t| t.text.as_str()).collect::<Vec<_>>(),
///            vec!["Hello", "world"]);
/// ```
pub fn tokenize(text: &str, unit: TokenUnit) -> Result<Vec<Token>, NLError> {
    let text_c =
        CString::new(text).map_err(|e| NLError::InvalidArgument(format!("text NUL byte: {e}")))?;
    let mut array: *mut c_void = ptr::null_mut();
    let mut count: usize = 0;
    let status = unsafe {
        ffi::nl_tokenize(
            text_c.as_ptr(),
            unit as i32,
            &mut array,
            &mut count,
        )
    };
    if status != ffi::status::OK {
        return Err(NLError::Unknown {
            code: status,
            message: "tokenization failed".into(),
        });
    }
    if array.is_null() || count == 0 {
        return Ok(Vec::new());
    }
    let typed = array.cast::<ffi::TokenRaw>();
    let mut v = Vec::with_capacity(count);
    for i in 0..count {
        let raw = unsafe { &*typed.add(i) };
        let text = if raw.text.is_null() {
            String::new()
        } else {
            unsafe { core::ffi::CStr::from_ptr(raw.text) }
                .to_string_lossy()
                .into_owned()
        };
        v.push(Token {
            start: raw.start,
            length: raw.length,
            text,
        });
    }
    unsafe { ffi::nl_tokens_free(array, count) };
    Ok(v)
}
