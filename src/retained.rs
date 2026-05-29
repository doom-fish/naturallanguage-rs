//! Declarative macro for retain/release wrapper boilerplate.
//!
//! Many wrapper types hold a single `NonNull<c_void>` `handle` to a retained
//! Objective-C / Swift object and hand-roll identical `Clone` (retain) and
//! `Drop` (release) implementations. `nl_retained!` consolidates that
//! boilerplate into a single audited place.
//!
//! The generated impls preserve the exact behavior of the previous
//! hand-written versions:
//! - `Drop` calls the supplied `release` FFI fn on `self.handle.as_ptr()`.
//! - `Clone` calls the supplied `retain` FFI fn and wraps the returned pointer
//!   in `NonNull::new(...).expect(...)`, matching the original null check.
//!
//! Types whose `Clone`/`Drop` carry extra logic beyond retain/release are
//! intentionally left hand-written.

/// Generate `Clone` and/or `Drop` impls for a `handle: NonNull<c_void>`
/// retain/release wrapper.
///
/// Variants:
/// - Drop only:
///   `nl_retained!(Ty, release = path::release);`
/// - Clone + Drop:
///   `nl_retained!(Ty, retain = path::retain, release = path::release);`
macro_rules! nl_retained {
    ($ty:ty, retain = $retain:path, release = $release:path $(,)?) => {
        impl Clone for $ty {
            fn clone(&self) -> Self {
                let handle = unsafe { $retain(self.handle.as_ptr()) };
                Self {
                    handle: ::std::ptr::NonNull::new(handle).expect("retain returned null"),
                }
            }
        }

        nl_retained!($ty, release = $release);
    };

    ($ty:ty, release = $release:path $(,)?) => {
        impl Drop for $ty {
            fn drop(&mut self) {
                unsafe { $release(self.handle.as_ptr()) };
            }
        }
    };
}

pub(crate) use nl_retained;
