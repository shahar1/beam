use apache_beam::{
    coders::{Coder, Context},
    elem_types::ElemType,
    register_coders,
};
use bytes::Buf;
use serde::{Deserialize, Serialize};

#[test]
fn serde_custom_coder() {
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct MyElement {
        some_field: String,
    }

    impl ElemType for MyElement {}

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    struct MyCoder;

    register_coders!(MyCoder);

    impl Coder for MyCoder {
        const URN: &'static str = "beam:dofn:rustsdk:1.0:MyCoder"; // TODO auto-gen via #[derive(Coder)]

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

    let my_coder = MyCoder::default();

    let coder_proto = my_coder.to_proto(vec![]);
    // serialize `proto_coder` into binary format, send to runners and then to SDK harness, then deserialize back to `proto_coder` agin.

    let urn = coder_proto.spec.unwrap().urn;

    // let anon_coder = AnonymousCoder::from(coder_proto);

    let element = MyElement {
        some_field: "some_value".to_string(),
    };

    let mut encoded_element = vec![];
    encode_from_urn(&urn, &element, &mut encoded_element, &Context::WholeStream).unwrap();

    let decoded_element_dyn =
        decode_from_urn(&urn, &mut encoded_element.reader(), &Context::WholeStream).unwrap();

    let decoded_element = decoded_element_dyn
        .as_any()
        .downcast_ref::<MyElement>()
        .unwrap();

    assert_eq!(decoded_element, &element);
}
