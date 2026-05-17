//! `NLModel`, `NLModelConfiguration`, and a minimal `MLModel` helper.

use core::ffi::{c_char, c_void};
use std::ptr::{self, NonNull};

use crate::error::NLError;
use crate::ffi;
use crate::language::Language;
use crate::util::{
    cstring_arg, decode_string_array, decode_usize_array, status_error, take_string,
};

/// `NLModelType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ModelType {
    Classifier = 0,
    Sequence = 1,
}

/// Minimal retained `MLModel` wrapper used for `NLModel::from_core_ml_model`.
#[derive(Debug)]
pub struct CoreMlModel {
    handle: NonNull<c_void>,
}

unsafe impl Send for CoreMlModel {}
unsafe impl Sync for CoreMlModel {}

impl Drop for CoreMlModel {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl CoreMlModel {
    pub fn from_source_path(path: impl AsRef<std::path::Path>) -> Result<Self, NLError> {
        let path_c = cstring_arg(&path.as_ref().to_string_lossy(), "path")?;
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_coreml_model_create_from_source_path(path_c.as_ptr(), &mut handle, &mut error)
        };
        if status != ffi::status::OK {
            return Err(status_error(status, "failed to load Core ML model", error));
        }
        let handle = NonNull::new(handle).ok_or_else(|| NLError::Unknown {
            code: ffi::status::UNKNOWN,
            message: "Core ML returned a null model".into(),
        })?;
        Ok(Self { handle })
    }

    pub fn from_compiled_path(path: impl AsRef<std::path::Path>) -> Result<Self, NLError> {
        let path_c = cstring_arg(&path.as_ref().to_string_lossy(), "path")?;
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_coreml_model_create_from_compiled_path(path_c.as_ptr(), &mut handle, &mut error)
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "failed to load compiled Core ML model",
                error,
            ));
        }
        let handle = NonNull::new(handle).ok_or_else(|| NLError::Unknown {
            code: ffi::status::UNKNOWN,
            message: "Core ML returned a null model".into(),
        })?;
        Ok(Self { handle })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

/// `NLModelConfiguration` metadata.
#[derive(Debug)]
pub struct ModelConfiguration {
    handle: NonNull<c_void>,
}

unsafe impl Send for ModelConfiguration {}
unsafe impl Sync for ModelConfiguration {}

impl Drop for ModelConfiguration {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl ModelConfiguration {
    pub(crate) const unsafe fn from_retained_ptr(handle: NonNull<c_void>) -> Self {
        Self { handle }
    }

    #[must_use]
    pub fn model_type(&self) -> ModelType {
        match unsafe { ffi::nl_model_configuration_type(self.handle.as_ptr()) } {
            1 => ModelType::Sequence,
            _ => ModelType::Classifier,
        }
    }

    pub fn language(&self) -> Result<Option<Language>, NLError> {
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_model_configuration_language(self.handle.as_ptr(), &mut out, &mut error)
        };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) }.map(Language::from))
        } else {
            Err(status_error(
                status,
                "model configuration language query failed",
                error,
            ))
        }
    }

    #[must_use]
    pub fn revision(&self) -> usize {
        unsafe { ffi::nl_model_configuration_revision(self.handle.as_ptr()) }
    }

    pub fn supported_revisions_for_type(model_type: ModelType) -> Result<Vec<usize>, NLError> {
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let status = unsafe {
            ffi::nl_model_supported_revisions_for_type(model_type as i32, &mut array, &mut count)
        };
        if status == ffi::status::OK {
            Ok(unsafe { decode_usize_array(array, count) })
        } else {
            Err(NLError::Unknown {
                code: status,
                message: "supported revisions query failed".into(),
            })
        }
    }

    #[must_use]
    pub fn current_revision_for_type(model_type: ModelType) -> usize {
        unsafe { ffi::nl_model_current_revision_for_type(model_type as i32) }
    }
}

/// A custom `NaturalLanguage` classifier or sequence model.
#[derive(Debug)]
pub struct Model {
    handle: NonNull<c_void>,
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe { ffi::nl_object_release(self.handle.as_ptr()) };
    }
}

impl Model {
    pub(crate) const unsafe fn from_retained_ptr(handle: NonNull<c_void>) -> Self {
        Self { handle }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, NLError> {
        let path_c = cstring_arg(&path.as_ref().to_string_lossy(), "path")?;
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::nl_model_with_contents_of_url(path_c.as_ptr(), &mut handle, &mut error) };
        if status != ffi::status::OK {
            return Err(status_error(status, "failed to load NLModel", error));
        }
        let handle = NonNull::new(handle).ok_or_else(|| NLError::Unknown {
            code: ffi::status::UNKNOWN,
            message: "NaturalLanguage returned a null model".into(),
        })?;
        Ok(Self { handle })
    }

    pub fn from_core_ml_model(model: &CoreMlModel) -> Result<Self, NLError> {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::nl_model_with_mlmodel(model.as_ptr(), &mut handle, &mut error) };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "failed to create NLModel from MLModel",
                error,
            ));
        }
        let handle = NonNull::new(handle).ok_or_else(|| NLError::Unknown {
            code: ffi::status::UNKNOWN,
            message: "NaturalLanguage returned a null model".into(),
        })?;
        Ok(Self { handle })
    }

    pub fn configuration(&self) -> Result<ModelConfiguration, NLError> {
        let handle = NonNull::new(unsafe { ffi::nl_model_configuration(self.handle.as_ptr()) })
            .ok_or_else(|| NLError::Unknown {
                code: ffi::status::UNKNOWN,
                message: "failed to get model configuration".into(),
            })?;
        Ok(unsafe { ModelConfiguration::from_retained_ptr(handle) })
    }

    pub fn predicted_label_for_string(&self, text: &str) -> Result<Option<String>, NLError> {
        let text_c = cstring_arg(text, "text")?;
        let mut out: *mut c_char = ptr::null_mut();
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_model_predicted_label_for_string(
                self.handle.as_ptr(),
                text_c.as_ptr(),
                &mut out,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(unsafe { take_string(out) })
        } else {
            Err(status_error(
                status,
                "predicted_label_for_string failed",
                error,
            ))
        }
    }

    pub fn predicted_labels_for_tokens(&self, tokens: &[String]) -> Result<Vec<String>, NLError> {
        let token_cs = tokens
            .iter()
            .map(|token| cstring_arg(token, "token"))
            .collect::<Result<Vec<_>, _>>()?;
        let token_ptrs = token_cs
            .iter()
            .map(|token| token.as_ptr())
            .collect::<Vec<_>>();
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_model_predicted_labels_for_tokens(
                self.handle.as_ptr(),
                token_ptrs.as_ptr(),
                token_ptrs.len(),
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "predicted_labels_for_tokens failed",
                error,
            ));
        }
        Ok(unsafe { decode_string_array(array, count) })
    }

    pub fn predicted_label_hypotheses_for_string(
        &self,
        text: &str,
        maximum_count: usize,
    ) -> Result<Vec<(String, f64)>, NLError> {
        let text_c = cstring_arg(text, "text")?;
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_model_predicted_label_hypotheses_for_string(
                self.handle.as_ptr(),
                text_c.as_ptr(),
                maximum_count,
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "predicted_label_hypotheses_for_string failed",
                error,
            ));
        }
        Ok(unsafe { decode_label_scores(array, count) })
    }

    pub fn predicted_label_hypotheses_for_tokens(
        &self,
        tokens: &[String],
        maximum_count: usize,
    ) -> Result<Vec<Vec<(String, f64)>>, NLError> {
        let token_cs = tokens
            .iter()
            .map(|token| cstring_arg(token, "token"))
            .collect::<Result<Vec<_>, _>>()?;
        let token_ptrs = token_cs
            .iter()
            .map(|token| token.as_ptr())
            .collect::<Vec<_>>();
        let mut array: *mut c_void = ptr::null_mut();
        let mut count: usize = 0;
        let mut error: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::nl_model_predicted_label_hypotheses_for_tokens(
                self.handle.as_ptr(),
                token_ptrs.as_ptr(),
                token_ptrs.len(),
                maximum_count,
                &mut array,
                &mut count,
                &mut error,
            )
        };
        if status != ffi::status::OK {
            return Err(status_error(
                status,
                "predicted_label_hypotheses_for_tokens failed",
                error,
            ));
        }
        Ok(unsafe { decode_hypothesis_sets(array, count) })
    }
}

unsafe fn decode_label_scores(array: *mut c_void, count: usize) -> Vec<(String, f64)> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::StringDoubleRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        let raw = &*typed.add(idx);
        let label = if raw.key.is_null() {
            String::new()
        } else {
            core::ffi::CStr::from_ptr(raw.key)
                .to_string_lossy()
                .into_owned()
        };
        values.push((label, raw.value));
    }
    ffi::nl_string_doubles_free(array, count);
    values
}

unsafe fn decode_hypothesis_sets(array: *mut c_void, count: usize) -> Vec<Vec<(String, f64)>> {
    if array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::HypothesisSetRaw>();
    let mut values = Vec::with_capacity(count);
    for idx in 0..count {
        let raw = &*typed.add(idx);
        values.push(decode_label_scores(raw.entries, raw.count));
    }
    ffi::nl_hypothesis_sets_free(array, count);
    values
}
