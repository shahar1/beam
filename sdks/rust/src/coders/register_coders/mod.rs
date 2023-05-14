/// Must be called in outside of the main() function.
///
/// TODO example
#[macro_export]
macro_rules! register_coders {
    ($($coder:ident),*) => {
        fn encode_from_urn(urn: &str, elem: &dyn $crate::elem_types::ElemType, writer: &mut dyn std::io::Write, context: &$crate::coders::Context) -> Result<usize, std::io::Error> {
            match urn {
                $($coder::URN => $coder.encode(elem, writer, context),)*
                _ => panic!("unknown urn"),
            }
        }

        fn decode_from_urn(urn: &str, reader: &mut dyn std::io::Read, context: &$crate::coders::Context) -> Result<Box<dyn $crate::elem_types::ElemType>, std::io::Error> {
            match urn {
                $($coder::URN => $coder.decode(reader, context),)*
                _ => panic!("unknown urn"),
            }
        }

        #[ctor::ctor]
        fn init_coders_from_urn() {
            $crate::worker::CODER_FROM_URN.set($crate::worker::CoderFromUrn {
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
