mod sdk_launcher {
    use apache_beam::{
        coders::{Coder, Context},
        elem_types::ElemType,
        proto::beam_api::pipeline as proto_pipeline,
        register_coders,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct MyElement {
        pub some_field: String,
    }

    impl ElemType for MyElement {}

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    struct MyCoder;

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

    register_coders!(MyCoder);

    pub fn launcher_register_coder_proto() -> proto_pipeline::Coder {
        // in the proto registration (in the pipeline construction)
        let my_coder = MyCoder::default();
        my_coder.to_proto(vec![])
    }
}

mod sdk_harness {
    use bytes::Buf;
    use std::io;

    use apache_beam::{
        coders::Context, elem_types::ElemType, proto::beam_api::pipeline as proto_pipeline,
        worker::CoderFromUrn,
    };

    fn receive_coder() -> proto_pipeline::Coder {
        // serialized coder is sent from the launcher
        super::sdk_launcher::launcher_register_coder_proto()
    }

    fn create_my_element() -> super::sdk_launcher::MyElement {
        // A PTransform (UDF) create an instance of MyElement
        super::sdk_launcher::MyElement {
            some_field: "some_value".to_string(),
        }
    }

    fn encode_element(element: &dyn ElemType, coder: &proto_pipeline::Coder) -> Vec<u8> {
        let urn = &coder.spec.as_ref().unwrap().urn;

        let mut encoded_element = vec![];
        CoderFromUrn::global()
            .encode_from_urn(urn, element, &mut encoded_element, &Context::WholeStream)
            .unwrap();

        encoded_element
    }

    fn decode_element(
        elem_reader: &mut dyn io::Read,
        coder: &proto_pipeline::Coder,
    ) -> Box<dyn ElemType> {
        let urn = &coder.spec.as_ref().unwrap().urn;

        let decoded_element_dyn = CoderFromUrn::global()
            .decode_from_urn(urn, elem_reader, &Context::WholeStream)
            .unwrap();

        decoded_element_dyn
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
