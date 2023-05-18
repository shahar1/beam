mod custom_coder_from_urn;
pub use custom_coder_from_urn::{CustomCoderFromUrn, CUSTOM_CODER_FROM_URN};

use crate::worker::coder_from_urn::preset_coder_from_urn::PresetCoderFromUrn;

mod preset_coder_from_urn;

pub(in crate::worker) struct CoderFromUrn;

impl CoderFromUrn {
    pub(in crate::worker) fn encode_from_urn(
        urn: &str,
        elem: &dyn crate::elem_types::ElemType,
        writer: &mut dyn std::io::Write,
        context: &crate::coders::Context,
    ) -> Result<usize, std::io::Error> {
        PresetCoderFromUrn::encode_from_urn(urn, elem, writer, context).unwrap_or_else(|| {
            let custom = CustomCoderFromUrn::global();
            (custom.enc)(urn, elem, writer, context)
        })
    }

    pub(in crate::worker) fn decode_from_urn(
        urn: &str,
        reader: &mut dyn std::io::Read,
        context: &crate::coders::Context,
    ) -> Result<Box<dyn crate::elem_types::ElemType>, std::io::Error> {
        PresetCoderFromUrn::decode_from_urn(urn, reader, context).unwrap_or_else(|| {
            let custom = CustomCoderFromUrn::global();
            (custom.dec)(urn, reader, context)
        })
    }
}
