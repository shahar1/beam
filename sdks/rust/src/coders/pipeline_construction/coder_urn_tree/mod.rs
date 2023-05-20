use crate::proto::pipeline_v1;

/// A coder's URN and URNs of its component coders.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CoderUrnTree {
    pub(crate) coder_urn: &'static str,
    pub(crate) component_coder_urns: Vec<Self>,
}

impl CoderUrnTree {
    /// Recursively construct a `CoderUrnTree`.
    pub(crate) fn from_proto(root_coder_id: &str, pipeline_proto: &pipeline_v1::Pipeline) -> Self {
        todo!()
    }
}
