//! Text tagging — wraps `NLTagger`, `NLTagScheme`, and `NLTag`.

use core::ffi::{c_char, c_void};
use std::collections::BTreeMap;
use std::ffi::CString;
use std::ptr::{self, NonNull};

use crate::error::NLError;
use crate::ffi;
use crate::language::Language;
use crate::script::Script;
use crate::string_enum::string_extensible_enum;
use crate::types::TextRange;
use crate::util::{cstring_arg, decode_string_array, status_error, take_string};

#[cfg(feature = "gazetteer")]
use crate::gazetteer::Gazetteer;
#[cfg(feature = "model")]
use crate::model::Model;

string_extensible_enum! {
    /// A `NaturalLanguage` tag scheme.
    pub struct TagScheme {
        TOKEN_TYPE = "TokenType";
        LEXICAL_CLASS = "LexicalClass";
        NAME_TYPE = "NameType";
        NAME_TYPE_OR_LEXICAL_CLASS = "NameTypeOrLexicalClass";
        LEMMA = "Lemma";
        LANGUAGE = "Language";
        SCRIPT = "Script";
        SENTIMENT_SCORE = "SentimentScore";
    }
}

string_extensible_enum! {
    /// A `NaturalLanguage` tag value.
    pub struct Tag {
        WORD = "Word";
        PUNCTUATION = "Punctuation";
        WHITESPACE = "Whitespace";
        OTHER = "Other";
        NOUN = "Noun";
        VERB = "Verb";
        ADJECTIVE = "Adjective";
        ADVERB = "Adverb";
        PRONOUN = "Pronoun";
        DETERMINER = "Determiner";
        PARTICLE = "Particle";
        PREPOSITION = "Preposition";
        NUMBER = "Number";
        CONJUNCTION = "Conjunction";
        INTERJECTION = "Interjection";
        CLASSIFIER = "Classifier";
        IDIOM = "Idiom";
        OTHER_WORD = "OtherWord";
        SENTENCE_TERMINATOR = "SentenceTerminator";
        OPEN_QUOTE = "OpenQuote";
        CLOSE_QUOTE = "CloseQuote";
        OPEN_PARENTHESIS = "OpenParenthesis";
        CLOSE_PARENTHESIS = "CloseParenthesis";
        WORD_JOINER = "WordJoiner";
        DASH = "Dash";
        OTHER_PUNCTUATION = "OtherPunctuation";
        PARAGRAPH_BREAK = "ParagraphBreak";
        OTHER_WHITESPACE = "OtherWhitespace";
        PERSONAL_NAME = "PersonalName";
        PLACE_NAME = "PlaceName";
        ORGANIZATION_NAME = "OrganizationName";
    }
}

/// Options controlling which tokens `NLTagger` returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TaggerOptions(u64);

impl TaggerOptions {
    pub const NONE: Self = Self(0);
    pub const OMIT_WORDS: Self = Self(1 << 0);
    pub const OMIT_PUNCTUATION: Self = Self(1 << 1);
    pub const OMIT_WHITESPACE: Self = Self(1 << 2);
    pub const OMIT_OTHER: Self = Self(1 << 3);
    pub const JOIN_NAMES: Self = Self(1 << 4);
    pub const JOIN_CONTRACTIONS: Self = Self(1 << 5);

    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for TaggerOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// Result of `NLTagger.requestAssetsForLanguage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TaggerAssetsResult {
    Available = 0,
    NotAvailable = 1,
    Error = 2,
}

/// Lightweight orthography description for `NLTagger::set_orthography`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Orthography {
    /// Optional dominant script (e.g. `Latn`).
    pub dominant_script: Option<Script>,
    /// Script -> allowed languages.
    pub language_map: BTreeMap<Script, Vec<Language>>,
}

impl Orthography {
    /// Create an empty orthography.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the dominant script.
    #[must_use]
    pub fn with_dominant_script(mut self, script: Script) -> Self {
        self.dominant_script = Some(script);
        self
    }

    /// Insert a script-to-languages mapping.
    pub fn insert(&mut self, script: Script, languages: Vec<Language>) {
        self.language_map.insert(script, languages);
    }
}

/// One returned tag span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaggedRange {
    /// UTF-16 token range.
    pub range: TextRange,
    /// Text covered by the range.
    pub text: String,
    /// Optional tag value.
    pub tag: Option<Tag>,
}

/// One hypothesis from `tag_hypotheses_at_index`.
#[derive(Debug, Clone, PartialEq)]
pub struct TagHypothesis {
    /// Candidate tag.
    pub tag: Tag,
    /// Confidence score.
    pub confidence: f64,
}

/// Apple's three name-type categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EntityKind {
    PersonalName,
    PlaceName,
    OrganizationName,
    Other(()),
}

impl EntityKind {
    pub(crate) fn from_tag(tag: &Tag) -> Self {
        match tag.as_str() {
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

impl NamedEntity {
    /// Convert to a shared `TextRange`.
    #[must_use]
    pub const fn range(&self) -> TextRange {
        TextRange::new(self.start, self.length)
    }
}

/// Stateful wrapper over Apple's `NLTagger`.
#[derive(Debug)]
pub struct Tagger {
    handle: NonNull<c_void>,
}

unsafe impl Send for Tagger {}
unsafe impl Sync for Tagger {}

impl Drop for Tagger {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl Tagger {
    /// Create a tagger configured with the supplied schemes.
    pub fn new(tag_schemes: &[TagScheme]) -> Result<Self, NLError> {
        let schemes = tag_schemes
            .iter()
            .map(|scheme| cstring_arg(scheme.as_str(), "tag scheme"))
            .collect::<Result<Vec<_>, _>>()?;
        let scheme_ptrs = schemes
            .iter()
            .map(|scheme| scheme.as_ptr())
            .collect::<Vec<_>>();
        let mut error: *mut c_char = ptr::null_mut();
        let handle = NonNull::new(unsafe {
            ffi::nl_tagger_create(scheme_ptrs.as_ptr(), scheme_ptrs.len(), &mut error)
        })
        .ok_or_else(|| status_error(ffi::status::UNKNOWN, "failed to create tagger", error))?;
        Ok(Self { handle })
    }

    /// Tag schemes configured on this tagger.
    pub fn tag_schemes(&self) -> Result<Vec<TagScheme>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_tag_schemes(self.handle.as_ptr(), &mut array, &mut count, &mut error)
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "tagger tag_schemes failed", error));
        }
        Ok(unsafe { decode_string_array(array, count) }
            .into_iter()
            .map(TagScheme::from)
            .collect())
    }

    /// Current tagger string.
    pub fn string(&self) -> Result<Option<String>, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::nl_tagger_string(self.handle.as_ptr(), &mut out, &mut error) };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) })
        } else {
            Err(status_error(status, "tagger string query failed", error))
        }
    }

    /// Replace the tagger string. Pass `None` to clear it.
    pub fn set_string(&mut self, text: Option<&str>) -> Result<(), NLError> {
        let text_c = text.map(|value| cstring_arg(value, "text")).transpose()?;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_set_string(
                self.handle.as_ptr(),
                text_c.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(status, "tagger string update failed", error))
        }
    }

    /// Available schemes for a given token unit and language.
    pub fn available_tag_schemes(
        unit: crate::tokenizer::TokenUnit,
        language: &Language,
    ) -> Result<Vec<TagScheme>, NLError> {
        let language_c = cstring_arg(language.as_str(), "language")?;
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_available_tag_schemes(
                unit as i32,
                language_c.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "available tag schemes query failed",
                error,
            ));
        }
        Ok(unsafe { decode_string_array(array, count) }
            .into_iter()
            .map(TagScheme::from)
            .collect())
    }

    /// Range of the token containing `character_index`.
    pub fn token_range_at_index(
        &self,
        character_index: usize,
        unit: crate::tokenizer::TokenUnit,
    ) -> Result<TextRange, NLError> {
        let mut range = ffi::TextRangeRaw::default();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_token_range_at_index(
                self.handle.as_ptr(),
                character_index,
                unit as i32,
                ptr::addr_of_mut!(range).cast(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(TextRange::new(range.start, range.length))
        } else {
            Err(status_error(
                status,
                "tagger token_range_at_index failed",
                error,
            ))
        }
    }

    /// Smallest token-aligned range intersecting `range`.
    pub fn token_range_for_range(
        &self,
        range: TextRange,
        unit: crate::tokenizer::TokenUnit,
    ) -> Result<TextRange, NLError> {
        let mut out = ffi::TextRangeRaw::default();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_token_range_for_range(
                self.handle.as_ptr(),
                range.start,
                range.length,
                unit as i32,
                ptr::addr_of_mut!(out).cast(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(TextRange::new(out.start, out.length))
        } else {
            Err(status_error(
                status,
                "tagger token_range_for_range failed",
                error,
            ))
        }
    }

    /// Dominant language for the current string.
    pub fn dominant_language(&self) -> Result<Option<Language>, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::nl_tagger_dominant_language(self.handle.as_ptr(), &mut out, &mut error) };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) }.map(Language::from))
        } else if status == ffi::status::NO_DOMINANT_LANGUAGE {
            Ok(None)
        } else {
            Err(status_error(
                status,
                "tagger dominant language failed",
                error,
            ))
        }
    }

    /// Eager tag enumeration for `range`.
    pub fn tags_in_range(
        &self,
        range: TextRange,
        unit: crate::tokenizer::TokenUnit,
        scheme: &TagScheme,
        options: TaggerOptions,
    ) -> Result<Vec<TaggedRange>, NLError> {
        let scheme_c = cstring_arg(scheme.as_str(), "tag scheme")?;
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_tags_in_range(
                self.handle.as_ptr(),
                range.start,
                range.length,
                unit as i32,
                scheme_c.as_ptr(),
                options.bits(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "tagger tags_in_range failed", error));
        }
        Ok(unsafe { decode_tagged_ranges(array, count) })
    }

    /// Rust callback wrapper around `enumerateTagsInRange:`.
    pub fn enumerate_tags_in_range<F>(
        &self,
        range: TextRange,
        unit: crate::tokenizer::TokenUnit,
        scheme: &TagScheme,
        options: TaggerOptions,
        mut callback: F,
    ) -> Result<(), NLError>
    where
        F: FnMut(&TaggedRange) -> bool,
    {
        for tagged in self.tags_in_range(range, unit, scheme, options)? {
            if !callback(&tagged) {
                break;
            }
        }
        Ok(())
    }

    /// Tag and token range for `character_index`.
    pub fn tag_at_index(
        &self,
        character_index: usize,
        unit: crate::tokenizer::TokenUnit,
        scheme: &TagScheme,
    ) -> Result<TaggedRange, NLError> {
        let scheme_c = cstring_arg(scheme.as_str(), "tag scheme")?;
        let mut raw = ffi::TagSpanRaw::default();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_tag_at_index(
                self.handle.as_ptr(),
                character_index,
                unit as i32,
                scheme_c.as_ptr(),
                ptr::addr_of_mut!(raw).cast(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(unsafe { tagged_range_from_raw(ptr::addr_of_mut!(raw).cast()) })
        } else {
            Err(status_error(status, "tagger tag_at_index failed", error))
        }
    }

    /// Multiple tag hypotheses for `character_index`.
    pub fn tag_hypotheses_at_index(
        &self,
        character_index: usize,
        unit: crate::tokenizer::TokenUnit,
        scheme: &TagScheme,
        maximum_count: usize,
    ) -> Result<(TextRange, Vec<TagHypothesis>), NLError> {
        let scheme_c = cstring_arg(scheme.as_str(), "tag scheme")?;
        let mut range = ffi::TextRangeRaw::default();
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_tag_hypotheses_at_index(
                self.handle.as_ptr(),
                character_index,
                unit as i32,
                scheme_c.as_ptr(),
                maximum_count,
                ptr::addr_of_mut!(range).cast(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "tagger tag_hypotheses_at_index failed",
                error,
            ));
        }
        Ok((TextRange::new(range.start, range.length), unsafe {
            decode_tag_hypotheses(array, count)
        }))
    }

    /// Hint the language for the given text range.
    pub fn set_language(&mut self, language: &Language, range: TextRange) -> Result<(), NLError> {
        let language_c = cstring_arg(language.as_str(), "language")?;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_set_language(
                self.handle.as_ptr(),
                language_c.as_ptr(),
                range.start,
                range.length,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(status, "tagger set_language failed", error))
        }
    }

    /// Hint the orthography for the given text range.
    pub fn set_orthography(
        &mut self,
        orthography: &Orthography,
        range: TextRange,
    ) -> Result<(), NLError> {
        let dominant_script = orthography
            .dominant_script
            .as_ref()
            .map(|script| cstring_arg(script.as_str(), "script"))
            .transpose()?;
        let mappings = orthography
            .language_map
            .iter()
            .flat_map(|(script, languages)| {
                languages.iter().map(move |language| (script, language))
            })
            .map(|(script, language)| {
                Ok(OrthographyCStringEntry {
                    script: cstring_arg(script.as_str(), "script")?,
                    language: cstring_arg(language.as_str(), "language")?,
                })
            })
            .collect::<Result<Vec<_>, NLError>>()?;
        let entries = mappings
            .iter()
            .map(|entry| ffi::OrthographyEntryRefRaw {
                script: entry.script.as_ptr(),
                language: entry.language.as_ptr(),
            })
            .collect::<Vec<_>>();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_set_orthography(
                self.handle.as_ptr(),
                dominant_script
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                entries.as_ptr().cast(),
                entries.len(),
                range.start,
                range.length,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(status, "tagger set_orthography failed", error))
        }
    }

    /// Attach custom models for `tag_scheme`.
    #[cfg(feature = "model")]
    pub fn set_models(&mut self, models: &[&Model], tag_scheme: &TagScheme) -> Result<(), NLError> {
        let scheme_c = cstring_arg(tag_scheme.as_str(), "tag scheme")?;
        let handles = models
            .iter()
            .map(|model| model.as_ptr())
            .collect::<Vec<_>>();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_set_models(
                self.handle.as_ptr(),
                handles.as_ptr(),
                handles.len(),
                scheme_c.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(status, "tagger set_models failed", error))
        }
    }

    /// Attached custom models for `tag_scheme`.
    #[cfg(feature = "model")]
    pub fn models_for_tag_scheme(&self, tag_scheme: &TagScheme) -> Result<Vec<Model>, NLError> {
        let scheme_c = cstring_arg(tag_scheme.as_str(), "tag scheme")?;
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_models_for_tag_scheme(
                self.handle.as_ptr(),
                scheme_c.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "tagger models_for_tag_scheme failed",
                error,
            ));
        }
        Ok(unsafe { decode_model_handles(array, count) })
    }

    /// Attach custom gazetteers for `tag_scheme`.
    #[cfg(feature = "gazetteer")]
    pub fn set_gazetteers(
        &mut self,
        gazetteers: &[&Gazetteer],
        tag_scheme: &TagScheme,
    ) -> Result<(), NLError> {
        let scheme_c = cstring_arg(tag_scheme.as_str(), "tag scheme")?;
        let handles = gazetteers
            .iter()
            .map(|gazetteer| gazetteer.as_ptr())
            .collect::<Vec<_>>();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_set_gazetteers(
                self.handle.as_ptr(),
                handles.as_ptr(),
                handles.len(),
                scheme_c.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(status_error(status, "tagger set_gazetteers failed", error))
        }
    }

    /// Attached custom gazetteers for `tag_scheme`.
    #[cfg(feature = "gazetteer")]
    pub fn gazetteers_for_tag_scheme(
        &self,
        tag_scheme: &TagScheme,
    ) -> Result<Vec<Gazetteer>, NLError> {
        let scheme_c = cstring_arg(tag_scheme.as_str(), "tag scheme")?;
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_gazetteers_for_tag_scheme(
                self.handle.as_ptr(),
                scheme_c.as_ptr(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "tagger gazetteers_for_tag_scheme failed",
                error,
            ));
        }
        Ok(unsafe { decode_gazetteer_handles(array, count) })
    }

    /// Request any missing on-device assets for a language/scheme combination.
    pub fn request_assets(
        language: &Language,
        tag_scheme: &TagScheme,
    ) -> Result<TaggerAssetsResult, NLError> {
        let language_c = cstring_arg(language.as_str(), "language")?;
        let scheme_c = cstring_arg(tag_scheme.as_str(), "tag scheme")?;
        let mut result = TaggerAssetsResult::Error as i32;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_tagger_request_assets(
                language_c.as_ptr(),
                scheme_c.as_ptr(),
                &mut result,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(match result {
                0 => TaggerAssetsResult::Available,
                1 => TaggerAssetsResult::NotAvailable,
                _ => TaggerAssetsResult::Error,
            })
        } else {
            Err(status_error(status, "tagger asset request failed", error))
        }
    }
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
    let mut tagger = Tagger::new(&[TagScheme::NAME_TYPE])?;
    tagger.set_string(Some(text))?;
    let range = TextRange::new(0, text.encode_utf16().count());
    tagger
        .tags_in_range(
            range,
            crate::tokenizer::TokenUnit::Word,
            &TagScheme::NAME_TYPE,
            TaggerOptions::OMIT_PUNCTUATION
                | TaggerOptions::OMIT_WHITESPACE
                | TaggerOptions::JOIN_NAMES,
        )
        .map(|tags| {
            tags.into_iter()
                .filter_map(|tagged| {
                    let tag = tagged.tag?;
                    let kind = EntityKind::from_tag(&tag);
                    (!matches!(kind, EntityKind::Other(()))).then_some(NamedEntity {
                        start: tagged.range.start,
                        length: tagged.range.length,
                        text: tagged.text,
                        kind,
                    })
                })
                .collect()
        })
}

struct OrthographyCStringEntry {
    script: CString,
    language: CString,
}

unsafe fn decode_tagged_ranges(array: *mut c_void, count: usize) -> Vec<TaggedRange> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::TagSpanRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        values.push(tagged_range_from_raw(typed.add(idx)));
    }
    ffi::nl_tag_spans_free(array, count);
    values
}

unsafe fn tagged_range_from_raw(raw: *mut ffi::TagSpanRaw) -> TaggedRange {
    let raw_ref = &mut *raw;
    let text = if raw_ref.text.is_null() {
        String::new()
    } else {
        core::ffi::CStr::from_ptr(raw_ref.text)
            .to_string_lossy()
            .into_owned()
    };
    let tag = if raw_ref.tag.is_null() {
        None
    } else {
        Some(Tag::from(
            core::ffi::CStr::from_ptr(raw_ref.tag)
                .to_string_lossy()
                .into_owned(),
        ))
    };
    let range = TextRange::new(raw_ref.start, raw_ref.length);
    ffi::nl_tag_span_clear(raw.cast());
    TaggedRange { range, text, tag }
}

unsafe fn decode_tag_hypotheses(array: *mut c_void, count: usize) -> Vec<TagHypothesis> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::StringDoubleRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        let raw = &*typed.add(idx);
        let tag = if raw.key.is_null() {
            String::new()
        } else {
            core::ffi::CStr::from_ptr(raw.key)
                .to_string_lossy()
                .into_owned()
        };
        values.push(TagHypothesis {
            tag: Tag::from(tag),
            confidence: raw.value,
        });
    }
    ffi::nl_string_doubles_free(array, count);
    values
}

#[cfg(feature = "model")]
unsafe fn decode_model_handles(array: *mut c_void, count: usize) -> Vec<Model> {
    decode_handle_vec(array, count)
        .into_iter()
        .map(|handle| Model::from_retained_ptr(handle))
        .collect()
}

#[cfg(feature = "gazetteer")]
unsafe fn decode_gazetteer_handles(array: *mut c_void, count: usize) -> Vec<Gazetteer> {
    decode_handle_vec(array, count)
        .into_iter()
        .map(|handle| Gazetteer::from_retained_ptr(handle))
        .collect()
}

unsafe fn decode_handle_vec(array: *mut c_void, count: usize) -> Vec<NonNull<c_void>> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<*mut c_void>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        if let Some(handle) = NonNull::new(*typed.add(idx)) {
            values.push(handle);
        }
    }
    ffi::nl_handle_array_free(array, count);
    values
}
