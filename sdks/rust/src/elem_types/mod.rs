pub mod kv;

use std::{any::Any, fmt};

use bytes::Bytes;

use crate::{
    coders::{
        required_coders::{BytesCoder, Iterable, IterableCoder, KVCoder},
        rust_coders::UnitCoder,
        standard_coders::{NullableCoder, StrUtf8Coder, VarIntCoder},
        Coder, CoderUrnTree,
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

/// Element types used in Beam pipelines (including PTransforms, PCollections, Coders, etc.)
///
/// Note that if you would like to use byte arrays as elements, you should use `bytes::Bytes` instead of `Vec<u8>`.
/// `Vec<u8>` is encoded/decoded by `IterableCoder`, which is slower than `BytesCoder`.
pub trait ElemType: AsAny + Send + Sync + 'static {
    // TODO just have associate type for default coder
    fn default_coder_urn() -> CoderUrnTree
    where
        Self: Sized;
}

impl<E> ElemType for Vec<E>
where
    E: ElemType + fmt::Debug,
{
    fn default_coder_urn() -> CoderUrnTree {
        IterableCoder::<E>::coder_urn_tree()
    }
}

impl ElemType for Bytes {
    fn default_coder_urn() -> CoderUrnTree {
        BytesCoder::coder_urn_tree()
    }
}

impl<K, V> ElemType for KV<K, V>
where
    K: ElemType,
    V: ElemType,
{
    fn default_coder_urn() -> CoderUrnTree {
        KVCoder::<KV<K, V>>::coder_urn_tree()
    }
}

impl<E: ElemType + fmt::Debug> ElemType for Iterable<E> {
    fn default_coder_urn() -> CoderUrnTree {
        IterableCoder::<E>::coder_urn_tree()
    }
}

impl ElemType for String {
    fn default_coder_urn() -> CoderUrnTree {
        StrUtf8Coder::coder_urn_tree()
    }
}

impl ElemType for i8 {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for i16 {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for i32 {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for i64 {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for isize {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for u8 {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for u16 {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for u32 {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for u64 {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}
impl ElemType for usize {
    fn default_coder_urn() -> CoderUrnTree {
        VarIntCoder::<Self>::coder_urn_tree()
    }
}

impl ElemType for () {
    fn default_coder_urn() -> CoderUrnTree {
        UnitCoder::coder_urn_tree()
    }
}

impl<E1: ElemType + fmt::Debug, E2: ElemType + fmt::Debug> ElemType for (E1, E2) {
    fn default_coder_urn() -> CoderUrnTree {
        todo!("TupleCoder?")
    }
}
impl<E1: ElemType + fmt::Debug, E2: ElemType + fmt::Debug, E3: ElemType + fmt::Debug> ElemType
    for (E1, E2, E3)
{
    fn default_coder_urn() -> CoderUrnTree {
        todo!("TupleCoder?")
    }
}
impl<
        E1: ElemType + fmt::Debug,
        E2: ElemType + fmt::Debug,
        E3: ElemType + fmt::Debug,
        E4: ElemType + fmt::Debug,
    > ElemType for (E1, E2, E3, E4)
{
    fn default_coder_urn() -> CoderUrnTree {
        todo!("TupleCoder?")
    }
}
impl<
        E1: ElemType + fmt::Debug,
        E2: ElemType + fmt::Debug,
        E3: ElemType + fmt::Debug,
        E4: ElemType + fmt::Debug,
        E5: ElemType + fmt::Debug,
    > ElemType for (E1, E2, E3, E4, E5)
{
    fn default_coder_urn() -> CoderUrnTree {
        todo!("TupleCoder?")
    }
}

impl<E: ElemType> ElemType for Option<E> {
    fn default_coder_urn() -> CoderUrnTree {
        NullableCoder::<E>::coder_urn_tree()
    }
}
