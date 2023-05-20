use crate::coders::{
    required_coders::BytesCoder, rust_coders::GeneralObjectCoder, standard_coders::StrUtf8Coder,
    urns::PresetCoderUrn, Coder,
};
use strum::IntoEnumIterator;

#[derive(Eq, PartialEq, Debug)]
pub(in crate::worker::coder_from_urn) struct PresetCoderFromUrn;

impl PresetCoderFromUrn {
    /// Returns `None` if the urn is not a preset coder urn.
    pub(in crate::worker) fn encode_from_urn(
        urn: &str,
        elem: &dyn crate::elem_types::ElemType,
        writer: &mut dyn std::io::Write,
        context: &crate::coders::Context,
    ) -> Option<Result<usize, std::io::Error>> {
        let opt_variant = PresetCoderUrn::iter().find(|variant| variant.as_str() == urn);

        opt_variant.map(|variant| match variant {
            PresetCoderUrn::Bytes => BytesCoder::default().encode(elem, writer, context),
            PresetCoderUrn::Kv => todo!("create full type including components (not only urn but also full proto maybe required"),
            PresetCoderUrn::Iterable => todo!("create full type including components (not only urn but also full proto maybe required"),
            PresetCoderUrn::Nullable => todo!("create full type including components (not only urn but also full proto maybe required"),
            PresetCoderUrn::StrUtf8 => StrUtf8Coder::default().encode(elem, writer, context),
            PresetCoderUrn::VarInt => todo!("create full type including components (not only urn but also full proto maybe required"),
            PresetCoderUrn::Unit => todo!("make UnitCoder"),
            PresetCoderUrn::GeneralObject => {
                GeneralObjectCoder::default().encode(elem, writer, context)
            }
        })
    }

    /// Returns `None` if the urn is not a preset coder urn.
    pub(in crate::worker) fn decode_from_urn(
        urn: &str,
        reader: &mut dyn std::io::Read,
        context: &crate::coders::Context,
    ) -> Option<Result<Box<dyn crate::elem_types::ElemType>, std::io::Error>> {
        let opt_variant = PresetCoderUrn::iter().find(|variant| variant.as_str() == urn);

        opt_variant.map(|variant| match variant {
            PresetCoderUrn::Bytes => BytesCoder::default().decode(reader, context),
            PresetCoderUrn::Kv => todo!("create full type including components (not only urn but also full proto maybe required"),
            PresetCoderUrn::Iterable => todo!("create full type including components (not only urn but also full proto maybe required"),
            PresetCoderUrn::Nullable => todo!("create full type including components (not only urn but also full proto maybe required"),
            PresetCoderUrn::StrUtf8 => StrUtf8Coder::default().decode(reader, context),
            PresetCoderUrn::VarInt => todo!("create full type including components (not only urn but also full proto maybe required"),
            PresetCoderUrn::Unit => todo!("make UnitCoder"),
            PresetCoderUrn::GeneralObject => GeneralObjectCoder::default().decode(reader, context),
        })
    }
}
