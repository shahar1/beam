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

        fn to_proto_from_urn(urn: &str) -> $crate::proto::pipeline::v1::Coder {
            use $crate::coders::CoderUrn;

            match urn {
                $($coder::URN => $coder::default().to_proto(vec![]),)* // TODO: rethink component_coder_ids parameter
                _ => panic!("unknown urn: {}", urn),
            }
        }

        #[ctor::ctor]
        fn init_custom_coder_from_urn() {
            $crate::worker::CUSTOM_CODER_FROM_URN.set($crate::worker::CustomCoderFromUrn {
                enc: encode_from_urn,
                dec: decode_from_urn,
                to_proto: to_proto_from_urn,
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

pub(crate) type ToProtoFromUrnFn = fn(&str) -> crate::proto::pipeline_v1::Coder;
