/// A coder's URN and URNs of its component coders.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CoderUrnTree {
    pub(crate) coder_urn: &'static str,
    pub(crate) component_coder_urns: Vec<Self>,
}
