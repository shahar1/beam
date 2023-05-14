use std::io;

use crate::{coders::Context, elem_types::ElemType, proto::beam_api::pipeline as proto_pipeline};

/// An anonymous coder is created from a proto `message Coder`.
///
/// It internally instantiate a concrete implementation of the `trait Coder` based on the `urn` field of the proto `message Coder`.
#[derive(Debug)]
pub(in crate::worker) struct AnonymousCoder {
    coder_urn: String,
}

impl AnonymousCoder {
    pub(in crate::worker) fn encode(
        &self,
        element: &dyn ElemType,
        writer: &mut dyn io::Write,
        context: &Context,
    ) -> Result<(), std::io::Error> {
        todo!()
    }

    pub(in crate::worker) fn decode<E>(
        &self,
        reader: &mut dyn io::Read,
        context: &Context,
    ) -> Result<E, io::Error>
    where
        E: ElemType,
    {
        todo!()
    }
}

impl From<proto_pipeline::Coder> for AnonymousCoder {
    fn from(coder: proto_pipeline::Coder) -> Self {
        Self {
            coder_urn: coder.spec.unwrap().urn, // TODO use TryFrom instead
        }
    }
}
