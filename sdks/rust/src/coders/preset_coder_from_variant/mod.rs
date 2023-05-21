use crate::coders::{
    required_coders::{BytesCoder, IterableCoder, KVCoder},
    rust_coders::{GeneralObjectCoder, UnitCoder},
    standard_coders::{NullableCoder, StrUtf8Coder},
    urns::PresetCoderUrn,
    Coder,
};

/// # Returns
///
/// `None` if the `urn` is not for any preset coder.
pub(crate) fn preset_coder_from_variant(
    variant: &PresetCoderUrn,
    component_coders: Vec<Box<dyn Coder>>,
) -> Box<dyn Coder> {
    let coder: Box<dyn Coder> = match variant {
        PresetCoderUrn::Bytes => Box::new(BytesCoder::new(vec![])),
        PresetCoderUrn::Kv => Box::new(KVCoder::new(component_coders)),
        PresetCoderUrn::Iterable => Box::new(IterableCoder::new(component_coders)),
        PresetCoderUrn::Nullable => Box::new(NullableCoder::new(component_coders)),
        PresetCoderUrn::StrUtf8 => Box::new(StrUtf8Coder::new(vec![])),
        PresetCoderUrn::VarInt => todo!("each specific N type by Coder.spec.payload?"),
        PresetCoderUrn::Unit => Box::new(UnitCoder::new(vec![])),
        PresetCoderUrn::GeneralObject => Box::new(GeneralObjectCoder::<String>::new(vec![])),
    };
    coder
}
