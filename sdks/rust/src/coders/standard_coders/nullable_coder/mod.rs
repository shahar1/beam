use std::fmt;
use std::io::{self, Read, Write};
use std::marker::PhantomData;

use crate::coders::{urns::*, CoderForPipeline, CoderUrn};
use crate::coders::{Coder, Context};
use crate::elem_types::{DefaultCoder, ElemType};

pub struct NullableCoder<E: ElemType> {
    _elem_coder: Box<dyn Coder>,

    _e: PhantomData<E>,
}

impl<E> CoderUrn for NullableCoder<E>
where
    E: ElemType,
{
    const URN: &'static str = NULLABLE_CODER_URN;
}

impl Coder for NullableCoder<()> {
    fn new(mut component_coders: Vec<Box<dyn Coder>>) -> Self
    where
        Self: Sized,
    {
        let elem_coder = component_coders
            .pop()
            .expect("1st component coder should be element coder");
        Self {
            _elem_coder: elem_coder,
            _e: PhantomData,
        }
    }

    fn encode(
        &self,
        _element: &dyn ElemType,
        _writer: &mut dyn Write,
        _context: &Context,
    ) -> Result<usize, io::Error> {
        todo!()
    }

    fn decode(
        &self,
        _reader: &mut dyn Read,
        _context: &Context,
    ) -> Result<Box<dyn ElemType>, io::Error> {
        todo!()
    }
}

impl<E> CoderForPipeline for NullableCoder<E>
where
    E: ElemType + DefaultCoder,
{
    fn component_coder_urns() -> Vec<crate::coders::CoderUrnTree> {
        vec![E::default_coder_urn()]
    }
}

impl<E> fmt::Debug for NullableCoder<E>
where
    E: ElemType,
{
    fn fmt(&self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_struct("NullableCoder")
            .field("urn", &Self::URN)
            .finish()
    }
}
