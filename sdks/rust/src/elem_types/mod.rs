pub mod kv;

use std::{any::Any, fmt};

use bytes::Bytes;

use crate::{
    coders::{
        required_coders::{BytesCoder, Iterable, IterableCoder, KVCoder},
        rust_coders::UnitCoder,
        standard_coders::{NullableCoder, StrUtf8Coder, VarIntCoder},
        CoderForPipeline, CoderUrnTree,
    },
    elem_types::kv::KV,
};

/// `&dyn ElemType` is often downcasted.
///
/// `AsAny` is a utility trait to make to convert `ElemType` into `Any`, in order to use `Any::downcast_ref()`.
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<E: ElemType> AsAny for E {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

/// The coder used for an element type.
pub trait DefaultCoder {
    type C: CoderForPipeline;

    fn default_coder_urn() -> CoderUrnTree
    where
        Self: Sized,
    {
        Self::C::coder_urn_tree()
    }
}

/// Element types used in Beam pipelines (including PTransforms, PCollections, Coders, etc.)
///
/// Note that if you would like to use byte arrays as elements, you should use `bytes::Bytes` instead of `Vec<u8>`.
/// `Vec<u8>` is encoded/decoded by `IterableCoder`, which is slower than `BytesCoder`.
pub trait ElemType: AsAny + Send + Sync + 'static {}

impl<E> ElemType for Vec<E> where E: ElemType {}
impl<E> DefaultCoder for Vec<E>
where
    E: ElemType + DefaultCoder + fmt::Debug,
{
    type C = IterableCoder<E>;
}

impl ElemType for Bytes {}
impl DefaultCoder for Bytes {
    type C = BytesCoder;
}

impl<K, V> ElemType for KV<K, V>
where
    K: ElemType,
    V: ElemType,
{
}
impl<K, V> DefaultCoder for KV<K, V>
where
    K: ElemType + DefaultCoder,
    V: ElemType + DefaultCoder,
{
    type C = KVCoder<KV<K, V>>;
}

impl<E: ElemType + fmt::Debug> ElemType for Iterable<E> {}
impl<E> DefaultCoder for Iterable<E>
where
    E: ElemType + DefaultCoder + fmt::Debug,
{
    type C = IterableCoder<E>;
}

impl ElemType for String {}
impl DefaultCoder for String {
    type C = StrUtf8Coder;
}

impl ElemType for i8 {}
impl DefaultCoder for i8 {
    type C = VarIntCoder<Self>;
}

impl ElemType for i16 {}
impl DefaultCoder for i16 {
    type C = VarIntCoder<Self>;
}

impl ElemType for i32 {}
impl DefaultCoder for i32 {
    type C = VarIntCoder<Self>;
}

impl ElemType for i64 {}
impl DefaultCoder for i64 {
    type C = VarIntCoder<Self>;
}

impl ElemType for isize {}
impl DefaultCoder for isize {
    type C = VarIntCoder<Self>;
}

impl ElemType for u8 {}
impl DefaultCoder for u8 {
    type C = VarIntCoder<Self>;
}

impl ElemType for u16 {}
impl DefaultCoder for u16 {
    type C = VarIntCoder<Self>;
}

impl ElemType for u32 {}
impl DefaultCoder for u32 {
    type C = VarIntCoder<Self>;
}

impl ElemType for u64 {}
impl DefaultCoder for u64 {
    type C = VarIntCoder<Self>;
}

impl ElemType for usize {}
impl DefaultCoder for usize {
    type C = VarIntCoder<Self>;
}

impl ElemType for () {}
impl DefaultCoder for () {
    type C = UnitCoder;
}

impl<E1: ElemType + fmt::Debug, E2: ElemType + fmt::Debug> ElemType for (E1, E2) {}
impl<E1: ElemType + fmt::Debug, E2: ElemType + fmt::Debug, E3: ElemType + fmt::Debug> ElemType
    for (E1, E2, E3)
{
}
impl<
        E1: ElemType + fmt::Debug,
        E2: ElemType + fmt::Debug,
        E3: ElemType + fmt::Debug,
        E4: ElemType + fmt::Debug,
    > ElemType for (E1, E2, E3, E4)
{
}
impl<
        E1: ElemType + fmt::Debug,
        E2: ElemType + fmt::Debug,
        E3: ElemType + fmt::Debug,
        E4: ElemType + fmt::Debug,
        E5: ElemType + fmt::Debug,
    > ElemType for (E1, E2, E3, E4, E5)
{
}

impl<E: ElemType> ElemType for Option<E> {}
impl<E> DefaultCoder for Option<E>
where
    E: ElemType + DefaultCoder,
{
    type C = NullableCoder<E>;
}
