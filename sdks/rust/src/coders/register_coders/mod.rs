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

        fn coder_from_urn(urn: &str, component_coders: Vec<Box<dyn $crate::coders::Coder>>) -> Box<dyn $crate::coders::Coder> {
            use $crate::coders::{
                CoderUrn, urns::PresetCoderUrn,
                required_coders::{BytesCoder, KVCoder, IterableCoder},
                standard_coders::{StrUtf8Coder, NullableCoder},
                rust_coders::{GeneralObjectCoder, UnitCoder},
            };
            use strum::IntoEnumIterator;

            let opt_preset = PresetCoderUrn::iter().find(|variant| variant.as_str() == urn);
            match opt_preset {
                Some(preset) => {
                    let coder: Box<dyn Coder> = match preset {
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
                None => {
                    match urn {
                        $($coder::URN => Box::new(<$coder>::new(component_coders)),)*
                        _ => panic!("unknown urn: {}", urn),
                    }
                }
            }
        }

        #[cfg(not(test))]
        #[ctor::ctor]
        fn init_custom_coder_from_urn() {
            $crate::worker::CUSTOM_CODER_FROM_URN.set($crate::worker::CustomCoderFromUrn {
                func: Some(coder_from_urn),
            }).expect("CUSTOM_CODER_FROM_URN singleton is already initialized");
        }
        #[cfg(test)]
        #[ctor::ctor]
        fn init_custom_coder_from_urn() {
            // always overwrite to the new function pointers, which the currently-executed test case defined via `register_coders!` macro.
            *$crate::worker::CUSTOM_CODER_FROM_URN.write().unwrap() = {
                let coder_from_urn = $crate::worker::CustomCoderFromUrn {
                    func: Some(coder_from_urn),
                };
                let boxed = Box::new(coder_from_urn);
                let static_ref = Box::leak(boxed); // use only in tests
                Some(static_ref)
            };
        }
    }
}

pub(crate) type CustomCoderFromUrnFn =
    fn(&str, Vec<Box<dyn crate::coders::Coder>>) -> Box<dyn crate::coders::Coder>;
