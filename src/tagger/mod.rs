//! Named-entity recognition — wraps `NLTagger` with the `.nameType` scheme.

use core::ffi::c_void;
use core::ptr;
use std::ffi::CString;

use crate::error::NLError;
use crate::ffi;

/// Apple's three name-type categories (other tags are filtered out by the
/// bridge — see the bridge source for the exact `NLTag` set we accept).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EntityKind {
    PersonalName,
    PlaceName,
    OrganizationName,
    Other(()),
}

impl EntityKind {
    pub(crate) fn from_tag(tag: &str) -> Self {
        match tag {
            "PersonalName" => Self::PersonalName,
            "PlaceName" => Self::PlaceName,
            "OrganizationName" => Self::OrganizationName,
            _ => Self::Other(()),
        }
    }
}

/// One detected named entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedEntity {
    /// UTF-16 character offset of the entity start.
    pub start: usize,
    /// UTF-16 character length of the entity.
    pub length: usize,
    /// The entity text (may span multiple words when joined by `joinNames`).
    pub text: String,
    /// The category Apple assigned.
    pub kind: EntityKind,
}

/// Run named-entity recognition over `text` and return every detected
/// person / place / organisation.
///
/// # Errors
///
/// Returns [`NLError::InvalidArgument`] for invalid input strings.
///
/// # Examples
///
/// ```rust,no_run
/// use naturallanguage::tagger::{named_entities, EntityKind};
///
/// let entities = named_entities(
///     "Tim Cook visited Apple Park in Cupertino last Tuesday."
/// ).unwrap();
/// assert!(entities.iter().any(|e| e.kind == EntityKind::PersonalName));
/// ```
pub fn named_entities(text: &str) -> Result<Vec<NamedEntity>, NLError> {
    let text_c =
        CString::new(text).map_err(|e| NLError::InvalidArgument(format!("text NUL byte: {e}")))?;
    let mut array: *mut c_void = ptr::null_mut();
    let mut count: usize = 0;
    let status = unsafe { ffi::nl_named_entities(text_c.as_ptr(), &mut array, &mut count) };
    if status != ffi::status::OK {
        return Err(NLError::Unknown {
            code: status,
            message: "named-entity recognition failed".into(),
        });
    }
    if array.is_null() || count == 0 {
        return Ok(Vec::new());
    }
    let typed = array.cast::<ffi::NamedEntityRaw>();
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
        let tag = if raw.tag.is_null() {
            String::new()
        } else {
            unsafe { core::ffi::CStr::from_ptr(raw.tag) }
                .to_string_lossy()
                .into_owned()
        };
        v.push(NamedEntity {
            start: raw.start,
            length: raw.length,
            text,
            kind: EntityKind::from_tag(&tag),
        });
    }
    unsafe { ffi::nl_named_entities_free(array, count) };
    Ok(v)
}
