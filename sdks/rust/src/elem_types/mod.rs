use std::fmt;

use crate::coders::required_coders::{Iterable, KV};

/// Element types used in Beam pipelines (including PTransforms, PCollections, Coders, etc.)
pub trait ElemType: Clone + Send + 'static {}

impl ElemType for Vec<u8> {}

impl<K, V> ElemType for KV<K, V>
where
    K: Clone + Send + 'static,
    V: Clone + Send + 'static,
{
}

impl<E: ElemType + fmt::Debug> ElemType for Iterable<E> {}

impl ElemType for String {}

impl ElemType for i8 {}
impl ElemType for i16 {}
impl ElemType for i32 {}
impl ElemType for i64 {}
impl ElemType for isize {}
impl ElemType for u8 {}
impl ElemType for u16 {}
impl ElemType for u32 {}
impl ElemType for u64 {}
impl ElemType for usize {}

impl ElemType for () {}
