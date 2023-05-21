use std::fmt;

use crate::coders::{Coder, CustomCoderFromUrnFn};

/// The visibility is `pub` because this is used internally from `register_coders!` macro.
#[cfg(not(test))]
pub static CUSTOM_CODER_FROM_URN: once_cell::sync::OnceCell<CustomCoderFromUrn> =
    once_cell::sync::OnceCell::new();

thread_local! {
    /// In test codes, OnceCell cannot be used because multiple test cases might instantiate this via `register_coders!`.
    #[cfg(test)]
    pub static CUSTOM_CODER_FROM_URN: std::sync::RwLock<Option<&'static CustomCoderFromUrn>> =
        std::sync::RwLock::new(None);
}

/// Function with `CustomCoderFromUrnFn` type is defined via `register_coders!` macro.
/// It can be defined in user-local module, so we need to capture the pointer of the function in `ctor` constructor also defined in the macro.
///
/// The visibility is `pub` because this is instantiated internally from `register_coders!` macro.
pub struct CustomCoderFromUrn {
    /// None if `register_coders!` is not called.
    pub func: Option<CustomCoderFromUrnFn>,
}

impl CustomCoderFromUrn {
    #[cfg(not(test))]
    pub(crate) fn global() -> &'static Self {
        CUSTOM_CODER_FROM_URN.get_or_init(|| Self { func: None })
    }
    #[cfg(test)]
    pub(crate) fn global() -> &'static Self {
        CUSTOM_CODER_FROM_URN.with(|c| {
            if c.read().unwrap().is_none() {
                // `register_coder!()` is not called
                *c.write().unwrap() = {
                    let custom_coder_from_urn = CustomCoderFromUrn { func: None };
                    let boxed = Box::new(custom_coder_from_urn);
                    let static_ref = Box::leak(boxed); // use only in tests
                    Some(static_ref)
                };
            }
            c.read().unwrap().unwrap()
        })
    }

    /// # Returns
    ///
    /// `None` if:
    ///
    /// - `register_coders!` is not called.
    /// - Unknown URN is found in `urn_tree`.
    pub(crate) fn custom_coder_from_urn(
        &self,
        urn: &str,
        component_coders: Vec<Box<dyn Coder>>,
    ) -> Option<Box<dyn Coder>> {
        self.func.and_then(|f| f(urn, component_coders))
    }
}

impl fmt::Debug for CustomCoderFromUrn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomCoderFromUrn").finish()
    }
}
