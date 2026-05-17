//! `NLGazetteer` bindings.

use core::ffi::{c_char, c_void};
use std::collections::BTreeMap;
use std::ptr::{self, NonNull};

use crate::error::NLError;
use crate::ffi;
use crate::language::Language;
use crate::util::{cstring_arg, status_error, take_string};

/// An `NLGazetteer` for label lookups over custom term dictionaries.
#[derive(Debug)]
pub struct Gazetteer {
    handle: NonNull<c_void>,
}

// SAFETY: Gazetteer wraps an Objective-C object handle from NaturalLanguage.framework,
// which is thread-safe. Rust holds exclusive ownership of the handle, and the framework's
// internal locking ensures thread-safe access.
unsafe impl Send for Gazetteer {}

// SAFETY: The underlying NLGazetteer object is thread-safe.
unsafe impl Sync for Gazetteer {}

impl Drop for Gazetteer {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl Gazetteer {
    pub(crate) const unsafe fn from_retained_ptr(handle: NonNull<c_void>) -> Self {
        Self { handle }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Load a gazetteer from a serialized file on disk.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, NLError> {
        let path_c = cstring_arg(&path.as_ref().to_string_lossy(), "path")?;
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_gazetteer_with_contents_of_url(path_c.as_ptr(), &mut handle, &mut error)
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "failed to load gazetteer", error));
        }
        let handle = NonNull::new(handle).ok_or_else(|| NLError::Unknown {
            code: ffi::status::UNKNOWN,
            message: "NaturalLanguage returned a null gazetteer".into(),
        })?;
        Ok(Self { handle })
    }

    /// Load a gazetteer from serialized bytes.
    pub fn from_data(data: &[u8]) -> Result<Self, NLError> {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_gazetteer_with_data(data.as_ptr(), data.len(), &mut handle, &mut error)
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "failed to load gazetteer data", error));
        }
        let handle = NonNull::new(handle).ok_or_else(|| NLError::Unknown {
            code: ffi::status::UNKNOWN,
            message: "NaturalLanguage returned a null gazetteer".into(),
        })?;
        Ok(Self { handle })
    }

    /// Build a gazetteer from a label -> terms dictionary.
    pub fn from_dictionary(
        dictionary: &BTreeMap<String, Vec<String>>,
        language: Option<&Language>,
    ) -> Result<Self, NLError> {
        let label_terms = dictionary
            .iter()
            .flat_map(|(label, terms)| terms.iter().map(move |term| (label, term)))
            .map(|(label, term)| Ok((cstring_arg(label, "label")?, cstring_arg(term, "term")?)))
            .collect::<Result<Vec<_>, NLError>>()?;
        let refs = label_terms
            .iter()
            .map(|(label, term)| ffi::LabelTermRefRaw {
                label: label.as_ptr(),
                term: term.as_ptr(),
            })
            .collect::<Vec<_>>();
        let language_c = language
            .map(|value| cstring_arg(value.as_str(), "language"))
            .transpose()?;
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_gazetteer_with_dictionary(
                refs.as_ptr().cast(),
                refs.len(),
                language_c
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut handle,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "failed to create gazetteer", error));
        }
        let handle = NonNull::new(handle).ok_or_else(|| NLError::Unknown {
            code: ffi::status::UNKNOWN,
            message: "NaturalLanguage returned a null gazetteer".into(),
        })?;
        Ok(Self { handle })
    }

    /// Label for an input string, if any.
    pub fn label_for_string(&self, text: &str) -> Result<Option<String>, NLError> {
        let text_c = cstring_arg(text, "text")?;
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_gazetteer_label_for_string(
                self.handle.as_ptr(),
                text_c.as_ptr(),
                &mut out,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) })
        } else {
            Err(status_error(status, "gazetteer label lookup failed", error))
        }
    }

    /// Associated gazetteer language, if any.
    pub fn language(&self) -> Result<Option<Language>, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::nl_gazetteer_language(self.handle.as_ptr(), &mut out, &mut error) };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) }.map(Language::from))
        } else {
            Err(status_error(
                status,
                "gazetteer language query failed",
                error,
            ))
        }
    }

    /// Serialized gazetteer bytes.
    pub fn data(&self) -> Result<Vec<u8>, NLError> {
        let mut bytes = ffi::BytesRaw {
            bytes: ptr::null_mut(),
            len: 0,
        };
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_gazetteer_data(
                self.handle.as_ptr(),
                ptr::addr_of_mut!(bytes).cast(),
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "gazetteer data query failed", error));
        }
        if bytes.bytes.is_null() || bytes.len == 0 {
            return Ok(Vec::new());
        }
        let result =
            unsafe { std::slice::from_raw_parts(bytes.bytes.cast::<u8>(), bytes.len).to_vec() };
        unsafe { ffi::nl_bytes_free(bytes.bytes) };
        Ok(result)
    }

    /// Write a serialized gazetteer to disk.
    pub fn write_dictionary(
        dictionary: &BTreeMap<String, Vec<String>>,
        language: Option<&Language>,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), NLError> {
        let label_terms = dictionary
            .iter()
            .flat_map(|(label, terms)| terms.iter().map(move |term| (label, term)))
            .map(|(label, term)| Ok((cstring_arg(label, "label")?, cstring_arg(term, "term")?)))
            .collect::<Result<Vec<_>, NLError>>()?;
        let refs = label_terms
            .iter()
            .map(|(label, term)| ffi::LabelTermRefRaw {
                label: label.as_ptr(),
                term: term.as_ptr(),
            })
            .collect::<Vec<_>>();
        let language_c = language
            .map(|value| cstring_arg(value.as_str(), "language"))
            .transpose()?;
        let path_c = cstring_arg(&path.as_ref().to_string_lossy(), "path")?;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_gazetteer_write_dictionary(
                refs.as_ptr().cast(),
                refs.len(),
                language_c
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                path_c.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(status, "writing gazetteer failed", error))
        }
    }
}
