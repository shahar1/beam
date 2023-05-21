use crate::{
    coders::{Coder, CoderForPipeline, CoderUrnTree, Context},
    elem_types::{DefaultCoder, ElemType},
    register_coders,
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
    fn new(_component_coders: Vec<Box<dyn Coder>>) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

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
