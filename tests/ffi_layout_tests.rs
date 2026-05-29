//! ABI layout assertions for the `#[repr(C)]` structs shared with the Swift bridge.
//!
//! These structs are marshalled across the Rust <-> Swift `@_cdecl` FFI boundary.
//! If their size or alignment ever drifts from what the Swift side expects, the
//! data marshalling silently corrupts. These tests pin the layout so accidental
//! field reordering / type changes are caught at `cargo test` time rather than as
//! runtime garbage.

use std::mem::{align_of, size_of};

use naturallanguage::ffi::{
    nl_verify_ffi_layout, BytesRaw, CoreMlModelRefRaw, EmbeddingNeighborRaw,
    EmbeddingVectorEntryRefRaw, HypothesisSetRaw, LabelTermRefRaw, LanguageHypothesisRaw,
    LanguageHypothesisRefRaw, NamedEntityRaw, OrthographyEntryRefRaw, StringDoubleRaw, StringRaw,
    TagSpanRaw, TextRangeRaw, TokenRaw, TokenSpanRaw, TokenVectorRaw,
};

macro_rules! assert_layout {
    ($ty:ty, $size:expr, $align:expr) => {
        assert_eq!(
            size_of::<$ty>(),
            $size,
            concat!(stringify!($ty), " size drifted")
        );
        assert_eq!(
            align_of::<$ty>(),
            $align,
            concat!(stringify!($ty), " alignment drifted")
        );
    };
}

#[test]
fn ffi_struct_layouts() {
    assert_layout!(TextRangeRaw, 16, 8);
    assert_layout!(LanguageHypothesisRaw, 16, 8);
    assert_layout!(LanguageHypothesisRefRaw, 16, 8);
    assert_layout!(StringRaw, 8, 8);
    assert_layout!(StringDoubleRaw, 16, 8);
    assert_layout!(TokenRaw, 24, 8);
    assert_layout!(TokenSpanRaw, 32, 8);
    assert_layout!(TagSpanRaw, 32, 8);
    assert_layout!(NamedEntityRaw, 32, 8);
    assert_layout!(BytesRaw, 16, 8);
    assert_layout!(EmbeddingNeighborRaw, 16, 8);
    assert_layout!(EmbeddingVectorEntryRefRaw, 24, 8);
    assert_layout!(LabelTermRefRaw, 16, 8);
    assert_layout!(OrthographyEntryRefRaw, 16, 8);
    assert_layout!(TokenVectorRaw, 32, 8);
    assert_layout!(HypothesisSetRaw, 16, 8);
    assert_layout!(CoreMlModelRefRaw, 0, 1);
}

/// Cross-language ABI check: asks the Swift bridge to verify that *its*
/// `MemoryLayout` (size/stride/alignment) for every shared FFI struct matches
/// the values pinned on the Rust side. A `false` return means the Rust and
/// Swift layouts genuinely disagree, which is a real ABI bug.
#[test]
fn ffi_layout_matches_swift() {
    // SAFETY: `nl_verify_ffi_layout` takes no arguments and only reads
    // compile-time `MemoryLayout` constants in the Swift bridge.
    let matches = unsafe { nl_verify_ffi_layout() };
    assert!(
        matches,
        "Swift FFI struct layout disagrees with Rust layout (ABI mismatch)"
    );
}
