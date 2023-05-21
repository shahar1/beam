mod coder_from_urn {
    use std::fmt;

    use bytes::Bytes;

    use crate::{
        coders::{
            urns::{BYTES_CODER_URN, ITERABLE_CODER_URN},
            Coder, CoderUrnTree, Context,
        },
        elem_types::ElemType,
        worker::CoderFromUrn,
    };

    fn assert_encode_decode<E: ElemType + PartialEq + fmt::Debug>(
        coder: Box<dyn Coder>,
        element: &E,
    ) {
        let mut encoded_element = vec![];
        coder
            .encode(element, &mut encoded_element, &Context::WholeStream)
            .unwrap();

        let mut encoded_element_reader = encoded_element.as_slice();

        let decoded_element_dyn = coder
            .decode(&mut encoded_element_reader, &Context::WholeStream)
            .unwrap();
        let decoded_element = decoded_element_dyn.as_any().downcast_ref::<E>().unwrap();

        assert_eq!(element, decoded_element);
    }

    fn t<E: ElemType + PartialEq + fmt::Debug>(coder_urn_tree: CoderUrnTree, element: E) {
        let coder = CoderFromUrn::global().coder_from_urn(&coder_urn_tree);
        assert_encode_decode(coder, &element)
    }

    #[test]
    fn preset_coder_without_components_success() {
        t(
            CoderUrnTree {
                coder_urn: BYTES_CODER_URN.to_string(),
                component_coder_urns: vec![],
            },
            Bytes::from("hello"),
        );
    }

    #[test]
    fn preset_coder_with_components_success() {
        t(
            CoderUrnTree {
                coder_urn: ITERABLE_CODER_URN.to_string(),
                component_coder_urns: vec![CoderUrnTree {
                    coder_urn: BYTES_CODER_URN.to_string(),
                    component_coder_urns: vec![],
                }],
            },
            vec![Bytes::from("hello"), Bytes::from("world")],
        );
    }

    #[test]
    #[should_panic]
    fn preset_coder_without_components_fail() {
        t(
            CoderUrnTree {
                coder_urn: BYTES_CODER_URN.to_string(),
                component_coder_urns: vec![],
            },
            42,
        );
    }
}

mod serde_preset_coder_test {
    mod sdk_launcher {
        use crate::{coders::CoderUrnTree, internals::pvalue::PValue, transforms::create::Create};

        pub fn launcher_register_coder_proto() -> CoderUrnTree {
            // in the proto registration (in the pipeline construction)
            let root = PValue::<()>::root();
            let pvalue = root.apply(Create::new(vec!["a".to_string(), "b".to_string()]));
            (&pvalue).into()
        }
    }

    mod sdk_harness {
        use bytes::Buf;
        use std::io;

        use crate::{
            coders::{CoderUrnTree, Context},
            elem_types::ElemType,
            worker::CoderFromUrn,
        };

        fn receive_coder() -> CoderUrnTree {
            // serialized coder is sent from the launcher
            super::sdk_launcher::launcher_register_coder_proto()
        }

        fn create_element() -> String {
            // A PTransform (UDF) create an instance of i32
            "hello".to_string()
        }

        fn encode_element(element: &dyn ElemType, coder_urn_tree: &CoderUrnTree) -> Vec<u8> {
            let mut encoded_element = vec![];

            let coder = CoderFromUrn::global().coder_from_urn(coder_urn_tree);
            coder
                .encode(element, &mut encoded_element, &Context::WholeStream)
                .unwrap();

            encoded_element
        }

        fn decode_element(
            elem_reader: &mut dyn io::Read,
            coder_urn_tree: &CoderUrnTree,
        ) -> Box<dyn ElemType> {
            let coder = CoderFromUrn::global().coder_from_urn(coder_urn_tree);
            coder.decode(elem_reader, &Context::WholeStream).unwrap()
        }

        pub fn test() {
            let coder = receive_coder();
            let element = create_element();

            let encoded_element = encode_element(&element, &coder);
            let decoded_element_dyn = decode_element(&mut encoded_element.reader(), &coder);

            let decoded_element = decoded_element_dyn
                .as_any()
                .downcast_ref::<String>()
                .unwrap();

            assert_eq!(decoded_element, &element);
        }
    }

    #[test]
    fn serde_custom_coder() {
        sdk_harness::test();
    }
}

mod serde_costom_coder_test {
    mod sdk_launcher {
        use crate::{
            coders::{Coder, CoderForPipeline, CoderUrnTree, Context},
            elem_types::{DefaultCoder, ElemType},
            internals::pvalue::PValue,
            register_coders,
            transforms::create::Create,
        };

        #[derive(Clone, PartialEq, Eq, Debug)]
        pub struct MyElement {
            pub some_field: String,
        }

        impl ElemType for MyElement {}

        impl DefaultCoder for MyElement {
            type C = MyCoder;
        }

        #[derive(Debug, Default)]
        pub struct MyCoder;

        impl Coder for MyCoder {
            fn encode(
                &self,
                element: &dyn ElemType,
                writer: &mut dyn std::io::Write,
                _context: &Context,
            ) -> Result<usize, std::io::Error> {
                let element = element.as_any().downcast_ref::<MyElement>().unwrap();

                writer
                    .write_all(format!("ENCPREFIX{}", element.some_field).as_bytes())
                    .map(|_| 0) // TODO make Result<usize, std::io::Error> to Result<(), std::io::Error>
            }

            fn decode(
                &self,
                reader: &mut dyn std::io::Read,
                _context: &Context,
            ) -> Result<Box<dyn ElemType>, std::io::Error> {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;

                let encoded_element = String::from_utf8(buf).unwrap();
                let element = encoded_element.strip_prefix("ENCPREFIX").unwrap();
                Ok(Box::new(MyElement {
                    some_field: element.to_string(),
                }))
            }
        }

        impl CoderForPipeline for MyCoder {
            fn component_coder_urns() -> Vec<CoderUrnTree> {
                vec![]
            }
        }

        register_coders!(MyCoder);

        pub fn launcher_register_coder_proto() -> CoderUrnTree {
            // in the proto registration (in the pipeline construction)
            let root = PValue::<()>::root();
            let pvalue = root.apply(Create::new(vec![
                MyElement {
                    some_field: "a".to_string(),
                },
                MyElement {
                    some_field: "b".to_string(),
                },
            ]));
            (&pvalue).into()
        }
    }

    mod sdk_harness {
        use bytes::Buf;
        use std::io;

        use crate::{
            coders::{CoderUrnTree, Context},
            elem_types::ElemType,
            worker::CoderFromUrn,
        };

        fn receive_coder() -> CoderUrnTree {
            // serialized coder is sent from the launcher
            super::sdk_launcher::launcher_register_coder_proto()
        }

        fn create_my_element() -> super::sdk_launcher::MyElement {
            // A PTransform (UDF) create an instance of MyElement
            super::sdk_launcher::MyElement {
                some_field: "some_value".to_string(),
            }
        }

        fn encode_element(element: &dyn ElemType, coder_urn_tree: &CoderUrnTree) -> Vec<u8> {
            let mut encoded_element = vec![];

            let coder = CoderFromUrn::global().coder_from_urn(coder_urn_tree);
            coder
                .encode(element, &mut encoded_element, &Context::WholeStream)
                .unwrap();

            encoded_element
        }

        fn decode_element(
            elem_reader: &mut dyn io::Read,
            coder_urn_tree: &CoderUrnTree,
        ) -> Box<dyn ElemType> {
            let coder = CoderFromUrn::global().coder_from_urn(coder_urn_tree);
            coder.decode(elem_reader, &Context::WholeStream).unwrap()
        }

        pub fn test() {
            let coder = receive_coder();
            let element = create_my_element();

            let encoded_element = encode_element(&element, &coder);
            let decoded_element_dyn = decode_element(&mut encoded_element.reader(), &coder);

            let decoded_element = decoded_element_dyn
                .as_any()
                .downcast_ref::<super::sdk_launcher::MyElement>()
                .unwrap();

            assert_eq!(decoded_element, &element);
        }
    }

    #[test]
    fn serde_custom_coder() {
        sdk_harness::test();
    }
}
