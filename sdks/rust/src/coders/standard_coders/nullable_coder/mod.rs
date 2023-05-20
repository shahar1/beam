use std::fmt;
use std::io::{self, Read, Write};

use crate::coders::{urns::*, CoderForPipeline, CoderUrn};
use crate::coders::{Coder, Context};
use crate::elem_types::{DefaultCoder, ElemType};

pub struct NullableCoder<E: ElemType> {
    _e: std::marker::PhantomData<E>,
}

impl<E> CoderUrn for NullableCoder<E>
where
    E: ElemType,
{
    const URN: &'static str = NULLABLE_CODER_URN;
}

impl<E> Coder for NullableCoder<E>
where
    E: ElemType,
{
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

impl<E> Default for NullableCoder<E>
where
    E: ElemType,
{
    fn default() -> Self {
        Self {
            _e: std::marker::PhantomData,
        }
    }
}
