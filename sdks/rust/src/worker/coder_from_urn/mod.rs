use std::fmt;

use crate::coders::{Coder, CoderFromUrnFn, CoderUrnTree};

/// The visibility is `pub` because this is used internally from `register_coders!` macro.
///
/// In test codes, multiple test cases might instantiate this via `register_coders!`.
/// To avoid it, we define different version of `CoderFromUrn::global()` for test compilation.
#[cfg(not(test))]
pub static CODER_FROM_URN: once_cell::sync::OnceCell<CoderFromUrn> =
    once_cell::sync::OnceCell::new();
/// In test codes, multiple test cases might instantiate this via `register_coders!`.
/// To avoid it, we define different version of `CoderFromUrn::global()` for test compilation.
#[cfg(test)]
pub static CODER_FROM_URN: std::sync::RwLock<Option<&'static CoderFromUrn>> =
    std::sync::RwLock::new(None);

/// Function with `CoderFromUrnFn` type is defined via `register_coders!` macro.
/// It can be defined in user-local module, so we need to capture the pointer of the function in `ctor` constructor also defined in the macro.
///
/// The visibility is `pub` because this is instantiated internally from `register_coders!` macro.
pub struct CoderFromUrn {
    pub func: CoderFromUrnFn,
}

impl CoderFromUrn {
    #[cfg(not(test))]
    pub(in crate::worker) fn global() -> &'static CoderFromUrn {
        CODER_FROM_URN
            .get()
            .expect("you might forget calling `register_coders!(CustomCoder1, CustomCoder2)`")
    }
    #[cfg(test)]
    pub(in crate::worker) fn global() -> &'static CoderFromUrn {
        CODER_FROM_URN.read().unwrap().unwrap()
    }

    pub(in crate::worker) fn coder_from_urn(&self, urn_tree: &CoderUrnTree) -> Box<dyn Coder> {
        (self.func)(urn_tree)
    }
}

impl fmt::Debug for CoderFromUrn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomCoderFromUrn").finish()
    }
}
