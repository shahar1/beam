use std::fmt;

use crate::coders::required_coders::{Iterable, KV};

/// Element types used in Beam pipelines (including PTransforms, PCollections, Coders, etc.)
pub trait ElemType: Send + 'static {}

impl ElemType for Vec<u8> {}

impl<K, V> ElemType for KV<K, V>
where
    K: Send + 'static,
    V: Send + 'static,
{
}

impl<E: ElemType + fmt::Debug> ElemType for Iterable<E> {}
