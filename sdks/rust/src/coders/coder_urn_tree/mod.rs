use crate::{elem_types::ElemType, internals::pvalue::PValue, proto::pipeline_v1};

/// A coder's URN and URNs of its component coders.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CoderUrnTree {
    pub(crate) coder_urn: String,
    pub(crate) component_coder_urns: Vec<Self>,
}

impl CoderUrnTree {
    /// Recursively construct a `CoderUrnTree`.
    pub(crate) fn from_proto(root_coder_id: &str, pipeline_proto: &pipeline_v1::Pipeline) -> Self {
        let components = pipeline_proto.components.as_ref().unwrap();

        let coder = components.coders.get(root_coder_id).unwrap();

        let coder_urn = coder
            .spec
            .as_ref()
            .expect("coders in Rust SDK should have spec field")
            .to_owned()
            .urn;

        let component_coder_urns: Vec<CoderUrnTree> = coder
            .component_coder_ids
            .iter()
            .map(|component_coder_id| Self::from_proto(component_coder_id, pipeline_proto))
            .collect();

        Self {
            coder_urn,
            component_coder_urns,
        }
    }
}

impl<E: ElemType> From<&PValue<E>> for CoderUrnTree {
    fn from(pvalue: &PValue<E>) -> Self {
        let pcoll_id = pvalue.get_id();

        let pipeline_proto = pvalue.get_pipeline_arc().get_proto();
        let pipeline_proto = pipeline_proto.lock().unwrap();

        let components = pipeline_proto.components.as_ref().unwrap();
        let pcoll = components.pcollections.get(&pcoll_id).unwrap();
        let coder_id = &pcoll.coder_id;

        Self::from_proto(coder_id, &pipeline_proto)
    }
}
