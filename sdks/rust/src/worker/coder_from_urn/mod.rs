use std::fmt;

use once_cell::sync::OnceCell;

use crate::coders::{DecodeFromUrnFn, EncodeFromUrnFn};

/// The visibility is `pub` because this is used internally from `register_coders!` macro.
pub static CODER_FROM_URN: OnceCell<CoderFromUrn> = OnceCell::new();

/// The visibility is `pub` because this is instantiated internally from `register_coders!` macro.
pub struct CoderFromUrn {
    pub enc: EncodeFromUrnFn,
    pub dec: DecodeFromUrnFn,
}

impl CoderFromUrn {
    pub fn global() -> &'static CoderFromUrn {
        CODER_FROM_URN
            .get()
            .expect("you might forget calling `register_coders!(CustomCoder1, CustomCoder2)`")
    }

    pub fn encode_from_urn(
        &self,
        urn: &str,
        elem: &dyn crate::elem_types::ElemType,
        writer: &mut dyn std::io::Write,
        context: &crate::coders::Context,
    ) -> Result<usize, std::io::Error> {
        (self.enc)(urn, elem, writer, context)
    }

    pub fn decode_from_urn(
        &self,
        urn: &str,
        reader: &mut dyn std::io::Read,
        context: &crate::coders::Context,
    ) -> Result<Box<dyn crate::elem_types::ElemType>, std::io::Error> {
        (self.dec)(urn, reader, context)
    }
}

impl fmt::Debug for CoderFromUrn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodersFromUrn").finish()
    }
}
