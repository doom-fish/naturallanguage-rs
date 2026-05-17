//! Tokenisation — wraps `NLTokenizer`.

use core::ffi::{c_char, c_void};
use std::ptr::{self, NonNull};

use crate::error::NLError;
use crate::ffi;
use crate::language::Language;
use crate::types::TextRange;
use crate::util::{cstring_arg, status_error, take_string};

/// Granularity of tokenisation. Maps directly onto Apple's `NLTokenUnit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TokenUnit {
    Word = 0,
    Sentence = 1,
    Paragraph = 2,
    Document = 3,
}

/// Bitflags returned by `NLTokenizer` for token content classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TokenizerAttributes(u64);

impl TokenizerAttributes {
    /// No attribute flags.
    pub const NONE: Self = Self(0);
    /// Token is numeric.
    pub const NUMERIC: Self = Self(1 << 0);
    /// Token is symbolic.
    pub const SYMBOLIC: Self = Self(1 << 1);
    /// Token is emoji.
    pub const EMOJI: Self = Self(1 << 2);

    /// Create from raw bits.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Return raw bit representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Returns `true` if all bits in `other` are set.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for TokenizerAttributes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// One eager token, preserved for the original `tokenize()` helper API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// UTF-16 character offset of the token start in the source.
    pub start: usize,
    /// UTF-16 character length of the token.
    pub length: usize,
    /// The token text itself.
    pub text: String,
}

impl Token {
    /// Convert to a shared `TextRange`.
    #[must_use]
    pub const fn range(&self) -> TextRange {
        TextRange::new(self.start, self.length)
    }
}

/// A rich token span returned by the object-style tokenizer API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSpan {
    /// UTF-16 token range.
    pub range: TextRange,
    /// Token text slice.
    pub text: String,
    /// Token attributes inferred by Apple.
    pub attributes: TokenizerAttributes,
}

/// Stateful wrapper over Apple's `NLTokenizer`.
#[derive(Debug)]
pub struct Tokenizer {
    handle: NonNull<c_void>,
}

// SAFETY: Tokenizer wraps an Objective-C object handle from NaturalLanguage.framework,
// which is thread-safe. Rust holds exclusive ownership of the handle, and the framework's
// internal locking ensures thread-safe access.
unsafe impl Send for Tokenizer {}

// SAFETY: The underlying NLTokenizer object is thread-safe.
unsafe impl Sync for Tokenizer {}

impl Drop for Tokenizer {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl Tokenizer {
    /// Create a tokenizer for the requested unit.
    pub fn new(unit: TokenUnit) -> Result<Self, NLError> {
        let handle =
            NonNull::new(unsafe { ffi::nl_tokenizer_create(unit as i32) }).ok_or_else(|| {
                NLError::Unknown {
                    code: ffi::status::UNKNOWN,
                    message: "failed to create tokenizer".into(),
                }
            })?;
        Ok(Self { handle })
    }

    /// Tokenizer unit.
    #[must_use]
    pub fn unit(&self) -> TokenUnit {
        match unsafe { ffi::nl_tokenizer_unit(self.handle.as_ptr()) } {
            1 => TokenUnit::Sentence,
            2 => TokenUnit::Paragraph,
            3 => TokenUnit::Document,
            _ => TokenUnit::Word,
        }
    }

    /// Current tokenizer string.
    pub fn string(&self) -> Result<Option<String>, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::nl_tokenizer_string(self.handle.as_ptr(), &mut out, &mut error) };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) })
        } else {
            Err(status_error(status, "tokenizer string query failed", error))
        }
    }

    /// Replace the tokenizer's current string. Pass `None` to clear it.
    pub fn set_string(&mut self, text: Option<&str>) -> Result<(), NLError> {
        let text_c = text.map(|value| cstring_arg(value, "text")).transpose()?;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tokenizer_set_string(
                self.handle.as_ptr(),
                text_c.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(
                status,
                "tokenizer string update failed",
                error,
            ))
        }
    }

    /// Hint the tokenizer with a known language.
    pub fn set_language(&mut self, language: &Language) -> Result<(), NLError> {
        let language_c = cstring_arg(language.as_str(), "language")?;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tokenizer_set_language(self.handle.as_ptr(), language_c.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(
                status,
                "tokenizer language update failed",
                error,
            ))
        }
    }

    /// Range of the token containing `character_index`.
    pub fn token_range_at_index(&self, character_index: usize) -> Result<TextRange, NLError> {
        let mut range = ffi::TextRangeRaw::default();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tokenizer_token_range_at_index(
                self.handle.as_ptr(),
                character_index,
                ptr::addr_of_mut!(range).cast(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(TextRange::new(range.start, range.length))
        } else {
            Err(status_error(
                status,
                "tokenizer token_range_at_index failed",
                error,
            ))
        }
    }

    /// Smallest token-aligned range intersecting `range`.
    pub fn token_range_for_range(&self, range: TextRange) -> Result<TextRange, NLError> {
        let mut out = ffi::TextRangeRaw::default();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tokenizer_token_range_for_range(
                self.handle.as_ptr(),
                range.start,
                range.length,
                ptr::addr_of_mut!(out).cast(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(TextRange::new(out.start, out.length))
        } else {
            Err(status_error(
                status,
                "tokenizer token_range_for_range failed",
                error,
            ))
        }
    }

    /// Token ranges intersecting `range`.
    pub fn token_ranges_for_range(&self, range: TextRange) -> Result<Vec<TextRange>, NLError> {
        self.tokens_in_range(range)
            .map(|tokens| tokens.into_iter().map(|token| token.range).collect())
    }

    /// Eager token enumeration for `range`.
    pub fn tokens_in_range(&self, range: TextRange) -> Result<Vec<TokenSpan>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tokenizer_tokens_in_range(
                self.handle.as_ptr(),
                range.start,
                range.length,
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "tokenizer tokens_in_range failed",
                error,
            ));
        }
        Ok(unsafe { decode_token_spans(array, count) })
    }

    /// Rust callback wrapper around `enumerateTokensInRange:`.
    pub fn enumerate_tokens_in_range<F>(
        &self,
        range: TextRange,
        mut callback: F,
    ) -> Result<(), NLError>
    where
        F: FnMut(&TokenSpan) -> bool,
    {
        for token in self.tokens_in_range(range)? {
            if !callback(&token) {
                break;
            }
        }
        Ok(())
    }
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
    let mut tokenizer = Tokenizer::new(unit)?;
    tokenizer.set_string(Some(text))?;
    let full_range = TextRange::new(0, text.encode_utf16().count());
    tokenizer.tokens_in_range(full_range).map(|tokens| {
        tokens
            .into_iter()
            .map(|token| Token {
                start: token.range.start,
                length: token.range.length,
                text: token.text,
            })
            .collect()
    })
}

unsafe fn decode_token_spans(array: *mut c_void, count: usize) -> Vec<TokenSpan> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::TokenSpanRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        let raw = &*typed.add(idx);
        let text = if raw.text.is_null() {
            String::new()
        } else {
            core::ffi::CStr::from_ptr(raw.text)
                .to_string_lossy()
                .into_owned()
        };
        values.push(TokenSpan {
            range: TextRange::new(raw.start, raw.length),
            text,
            attributes: TokenizerAttributes::from_bits(raw.attributes),
        });
    }
    ffi::nl_token_spans_free(array, count);
    values
}
