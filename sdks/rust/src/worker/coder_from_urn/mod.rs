mod custom_coder_from_urn;
pub use custom_coder_from_urn::{CustomCoderFromUrn, CUSTOM_CODER_FROM_URN};

use crate::coders::{preset_coder_from_variant, urns::PresetCoderUrn, Coder};

pub(crate) fn coder_from_urn(urn: &str, component_coders: Vec<Box<dyn Coder>>) -> Box<dyn Coder> {
    use strum::IntoEnumIterator;

    let opt_preset = PresetCoderUrn::iter().find(|variant| variant.as_str() == urn);
    match opt_preset {
        Some(preset) => preset_coder_from_variant(&preset, component_coders),
        None => CustomCoderFromUrn::global().custom_coder_from_urn(urn, component_coders).unwrap_or_else(|| panic!("coder with URN {} not found. Did you register the custom coder by `register_coder!()`?", urn)),
    }
}
