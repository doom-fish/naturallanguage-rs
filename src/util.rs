use core::ffi::{c_char, c_void};
use std::ffi::CString;

use crate::{error::NLError, ffi};

pub fn cstring_arg(value: &str, name: &str) -> Result<CString, NLError> {
    CString::new(value).map_err(|e| NLError::InvalidArgument(format!("{name} NUL byte: {e}")))
}

pub unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        let value = core::ffi::CStr::from_ptr(ptr)
            .to_string_lossy()
            .into_owned();
        ffi::nl_string_free(ptr);
        Some(value)
    }
}

pub fn status_error(code: i32, fallback: &str, error: *mut c_char) -> NLError {
    let message = unsafe { take_string(error) }.unwrap_or_else(|| fallback.to_string());
    match code {
        ffi::status::INVALID_ARGUMENT => NLError::InvalidArgument(message),
        ffi::status::UNSUPPORTED => NLError::Unsupported(message),
        _ => NLError::Unknown { code, message },
    }
}

pub unsafe fn decode_string_array(array: *mut c_void, count: usize) -> Vec<String> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::StringRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        let value = (*typed.add(idx)).value;
        values.push(if value.is_null() {
            String::new()
        } else {
            core::ffi::CStr::from_ptr(value)
                .to_string_lossy()
                .into_owned()
        });
    }
    ffi::nl_strings_free(array, count);
    values
}

pub unsafe fn decode_usize_array(array: *mut c_void, count: usize) -> Vec<usize> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<usize>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        values.push(*typed.add(idx));
    }
    ffi::nl_usizes_free(array, count);
    values
}
