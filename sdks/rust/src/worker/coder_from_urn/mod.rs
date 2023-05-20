use std::fmt;

use once_cell::sync::OnceCell;

use crate::coders::{Coder, CoderFromUrnFn, CoderUrnTree};

/// The visibility is `pub` because this is used internally from `register_coders!` macro.
pub static CODER_FROM_URN: OnceCell<CoderFromUrn> = OnceCell::new();

/// The visibility is `pub` because this is instantiated internally from `register_coders!` macro.
pub struct CoderFromUrn {
    pub func: CoderFromUrnFn,
}

impl CoderFromUrn {
    pub(in crate::worker) fn global() -> &'static CoderFromUrn {
        CODER_FROM_URN
            .get()
            .expect("you might forget calling `register_coders!(CustomCoder1, CustomCoder2)`")
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
