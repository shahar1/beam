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

        fn coder_from_urn(urn_tree: &$crate::coders::CoderUrnTree) -> Box<dyn $crate::coders::Coder> {
            use $crate::coders::{
                CoderUrn, urns::PresetCoderUrn,
                required_coders::BytesCoder,
                standard_coders::StrUtf8Coder,
                rust_coders::GeneralObjectCoder,
            };
            use strum::IntoEnumIterator;

            let urn = urn_tree.coder_urn.as_ref();

            let opt_preset_coder: Option<Box<dyn Coder>> = {
                let opt_variant = PresetCoderUrn::iter().find(|variant| variant.as_str() == urn);

                opt_variant.map(|variant| {
                    let coder: Box<dyn Coder> = match variant {
                        PresetCoderUrn::Bytes => Box::new(BytesCoder::new(vec![])),
                        PresetCoderUrn::Kv => todo!("create full type including components (not only urn but also full proto maybe required"),
                        PresetCoderUrn::Iterable => todo!("create full type including components (not only urn but also full proto maybe required"),
                        PresetCoderUrn::Nullable => todo!("create full type including components (not only urn but also full proto maybe required"),
                        PresetCoderUrn::StrUtf8 => Box::new(StrUtf8Coder::new(vec![])),
                        PresetCoderUrn::VarInt => todo!("create full type including components (not only urn but also full proto maybe required"),
                        PresetCoderUrn::Unit => todo!("make UnitCoder"),
                        PresetCoderUrn::GeneralObject => Box::new(GeneralObjectCoder::<String>::new(vec![])),
                    };
                    coder
                })
            };

            opt_preset_coder.unwrap_or_else(|| {
                match urn {
                    $($coder::URN => Box::<$coder>::default(),)*
                    _ => panic!("unknown urn: {}", urn),
                }
            })
        }

        #[cfg(not(test))]
        #[ctor::ctor]
        fn init_custom_coder_from_urn() {
            $crate::worker::CODER_FROM_URN.set($crate::worker::CoderFromUrn {
                func: coder_from_urn,
            }).expect("CODER_FROM_URN singleton is already initialized");
        }
        #[cfg(test)]
        #[ctor::ctor]
        fn init_custom_coder_from_urn() {
            // always overwrite to the new function pointers, which the currently-executed test case defined via `register_coders!` macro.
            *$crate::worker::CODER_FROM_URN.write().unwrap() = {
                let coder_from_urn = $crate::worker::CoderFromUrn {
                    func: coder_from_urn,
                };
                let boxed = Box::new(coder_from_urn);
                let static_ref = Box::leak(boxed); // use only in tests
                Some(static_ref)
            };
        }
    }
}

pub(crate) type CoderFromUrnFn = fn(&crate::coders::CoderUrnTree) -> Box<dyn crate::coders::Coder>;
