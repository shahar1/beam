/// Macro to register custom coders in order for SDK harnesses to use them via their URNs.
///
/// Must be called in outside of the main() function.
///
/// # Example
///
/// ```ignore
/// struct MyCustomCoder1;
/// impl Coder MyCustomCoder1 { /* ... */ }
///
/// // ...
///
/// register_coders!(MyCustomCoder1, MyCustomCoder2);
///
/// fn main() {
///    // ...
/// }
/// ```
///
/// # Related doc
///
/// [Design doc: Custom Coders for the Beam Rust SDK](https://docs.google.com/document/d/1tUb8EoajRkxLW3mrJZzx6xxGhoiUSRKwVuT2uxjAeIU/edit#heading=h.mgr8mrx81tnc)
#[macro_export]
macro_rules! register_coders {
    ($($coder:ident),*) => {
        $(
            impl $crate::coders::CoderUrn for $coder {
                const URN: &'static str = concat!("beam:coder:rustsdk:1.0:", stringify!($coder));
            }
        )*

        fn coder_from_urn(urn: &str) -> Box<dyn $crate::coders::Coder> {
            use $crate::coders::{
                CoderUrn, urns::PresetCoderUrn,
                required_coders::BytesCoder,
                standard_coders::StrUtf8Coder,
                rust_coders::GeneralObjectCoder,
            };
            use strum::IntoEnumIterator;

            let opt_preset_coder: Option<Box<dyn Coder>> = {
                let opt_variant = PresetCoderUrn::iter().find(|variant| variant.as_str() == urn);

                opt_variant.map(|variant| {
                    let coder: Box<dyn Coder> = match variant {
                        PresetCoderUrn::Bytes => Box::new(BytesCoder::default()),
                        PresetCoderUrn::Kv => todo!("create full type including components (not only urn but also full proto maybe required"),
                        PresetCoderUrn::Iterable => todo!("create full type including components (not only urn but also full proto maybe required"),
                        PresetCoderUrn::Nullable => todo!("create full type including components (not only urn but also full proto maybe required"),
                        PresetCoderUrn::StrUtf8 => Box::new(StrUtf8Coder::default()),
                        PresetCoderUrn::VarInt => todo!("create full type including components (not only urn but also full proto maybe required"),
                        PresetCoderUrn::Unit => todo!("make UnitCoder"),
                        PresetCoderUrn::GeneralObject => Box::new(GeneralObjectCoder::default()),
                    };
                    coder
                })
            };

            opt_preset_coder.unwrap_or_else(|| {
                match urn {
                    $($coder::URN => Box::new($coder::default()),)*
                    _ => panic!("unknown urn: {}", urn),
                }
            })
        }

        fn encode_from_urn(urn: &str, elem: &dyn $crate::elem_types::ElemType, writer: &mut dyn std::io::Write, context: &$crate::coders::Context) -> Result<usize, std::io::Error> {
            use $crate::coders::CoderUrn;

            match urn {
                $($coder::URN => $coder::default().encode(elem, writer, context),)*
                _ => panic!("unknown urn: {}", urn),
            }
        }

        fn decode_from_urn(urn: &str, reader: &mut dyn std::io::Read, context: &$crate::coders::Context) -> Result<Box<dyn $crate::elem_types::ElemType>, std::io::Error> {
            use $crate::coders::CoderUrn;

            match urn {
                $($coder::URN => $coder::default().decode(reader, context),)*
                _ => panic!("unknown urn: {}", urn),
            }
        }

        #[ctor::ctor]
        fn init_custom_coder_from_urn() {
            $crate::worker::CUSTOM_CODER_FROM_URN.set($crate::worker::CustomCoderFromUrn {
                enc: encode_from_urn,
                dec: decode_from_urn,
            }).unwrap();
        }
    }
}

pub(crate) type EncodeFromUrnFn = fn(
    &str,
    &dyn crate::elem_types::ElemType,
    &mut dyn std::io::Write,
    &crate::coders::Context,
) -> Result<usize, std::io::Error>;

pub(crate) type DecodeFromUrnFn =
    fn(
        &str,
        &mut dyn std::io::Read,
        &crate::coders::Context,
    ) -> Result<Box<dyn crate::elem_types::ElemType>, std::io::Error>;
