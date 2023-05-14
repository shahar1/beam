pub mod kv;

use std::fmt;

use crate::{coders::required_coders::Iterable, elem_types::kv::KV};

/// Element types used in Beam pipelines (including PTransforms, PCollections, Coders, etc.)
pub trait ElemType: Send + Sync + 'static {}

impl ElemType for Vec<u8> {}

impl<K, V> ElemType for KV<K, V>
where
    K: Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
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

impl<E1: ElemType, E2: ElemType> ElemType for (E1, E2) {}
impl<E1: ElemType, E2: ElemType, E3: ElemType> ElemType for (E1, E2, E3) {}
impl<E1: ElemType, E2: ElemType, E3: ElemType, E4: ElemType> ElemType for (E1, E2, E3, E4) {}
impl<E1: ElemType, E2: ElemType, E3: ElemType, E4: ElemType, E5: ElemType> ElemType
    for (E1, E2, E3, E4, E5)
{
}

impl<E: ElemType> ElemType for Option<E> {}
