use std::fmt;

use once_cell::sync::OnceCell;

use crate::coders::{DecodeFromUrnFn, EncodeFromUrnFn, ToProtoFromUrnFn};

/// The visibility is `pub` because this is used internally from `register_coders!` macro.
pub static CUSTOM_CODER_FROM_URN: OnceCell<CustomCoderFromUrn> = OnceCell::new();

/// The visibility is `pub` because this is instantiated internally from `register_coders!` macro.
pub struct CustomCoderFromUrn {
    pub enc: EncodeFromUrnFn,
    pub dec: DecodeFromUrnFn,
    pub to_proto: ToProtoFromUrnFn,
}

impl CustomCoderFromUrn {
    pub(in crate::worker::coder_from_urn) fn global() -> &'static CustomCoderFromUrn {
        CUSTOM_CODER_FROM_URN
            .get()
            .expect("you might forget calling `register_coders!(CustomCoder1, CustomCoder2)`")
    }

    pub(in crate::worker::coder_from_urn) fn encode_from_urn(
        &self,
        urn: &str,
        elem: &dyn crate::elem_types::ElemType,
        writer: &mut dyn std::io::Write,
        context: &crate::coders::Context,
    ) -> Result<usize, std::io::Error> {
        (self.enc)(urn, elem, writer, context)
    }

    pub(in crate::worker::coder_from_urn) fn decode_from_urn(
        &self,
        urn: &str,
        reader: &mut dyn std::io::Read,
        context: &crate::coders::Context,
    ) -> Result<Box<dyn crate::elem_types::ElemType>, std::io::Error> {
        (self.dec)(urn, reader, context)
    }

    pub(in crate::worker::coder_from_urn) fn to_proto_from_urn(
        &self,
        urn: &str,
    ) -> crate::proto::pipeline_v1::Coder {
        (self.to_proto)(urn)
    }
}

impl fmt::Debug for CustomCoderFromUrn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomCoderFromUrn").finish()
    }
}
