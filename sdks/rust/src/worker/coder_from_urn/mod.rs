use std::fmt;

use crate::coders::{Coder, CustomCoderFromUrnFn};

/// The visibility is `pub` because this is used internally from `register_coders!` macro.
///
/// In test codes, multiple test cases might instantiate this via `register_coders!`.
/// To avoid it, we define different version of `CoderFromUrn::global()` for test compilation.
#[cfg(not(test))]
pub static CUSTOM_CODER_FROM_URN: once_cell::sync::OnceCell<CustomCoderFromUrn> =
    once_cell::sync::OnceCell::new();
/// In test codes, multiple test cases might instantiate this via `register_coders!`.
/// To avoid it, we define different version of `CoderFromUrn::global()` for test compilation.
///
/// TODO should be thread-local
#[cfg(test)]
pub static CUSTOM_CODER_FROM_URN: std::sync::RwLock<Option<&'static CustomCoderFromUrn>> =
    std::sync::RwLock::new(None);

/// Function with `CoderFromUrnFn` type is defined via `register_coders!` macro.
/// It can be defined in user-local module, so we need to capture the pointer of the function in `ctor` constructor also defined in the macro.
///
/// The visibility is `pub` because this is instantiated internally from `register_coders!` macro.
pub struct CustomCoderFromUrn {
    /// None if `register_coders!` is not called.
    pub func: Option<CustomCoderFromUrnFn>,
}

impl CustomCoderFromUrn {
    #[cfg(not(test))]
    pub(in crate::worker) fn global() -> &'static Self {
        CUSTOM_CODER_FROM_URN.get_or_init(|| Self { func: None })
    }
    #[cfg(test)]
    pub(in crate::worker) fn global() -> &'static Self {
        CUSTOM_CODER_FROM_URN.read().unwrap().unwrap()
    }

    /// # Returns
    ///
    /// `None` if:
    ///
    /// - `register_coders!` is not called.
    /// - Unknown URN is found in `urn_tree`.
    pub(in crate::worker) fn custom_coder_from_urn(
        &self,
        urn: &str,
        component_coders: Vec<Box<dyn Coder>>,
    ) -> Option<Box<dyn Coder>> {
        self.func.map(|f| f(urn, component_coders))
    }
}

impl fmt::Debug for CustomCoderFromUrn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomCoderFromUrn").finish()
    }
}
