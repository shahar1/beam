use crate::coders::{CoderUrn, CoderUrnTree};

/// Coder's methods used in pipeline construction.
pub trait CoderForPipeline: CoderUrn {
    /// Coder URN of `Self` and its component coders.
    #[doc(hidden)]
    fn coder_urn_tree() -> CoderUrnTree {
        CoderUrnTree {
            coder_urn: Self::URN.to_string(),
            component_coder_urns: Self::component_coder_urns(),
        }
    }

    /// URNs of Component coders (internal coders like `T` in `ListCoder<T>`).
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn component_coder_urns() -> Vec<CoderUrnTree> {
    ///   vec![ComponentCoder1::coder_urn_tree(), ComponentCoder2::coder_urn_tree()]
    /// }
    /// ```
    fn component_coder_urns() -> Vec<CoderUrnTree>;
}
