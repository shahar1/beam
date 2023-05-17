pub mod kv;

use std::{any::Any, fmt};

use bytes::Bytes;

use crate::{
    coders::{
        required_coders::Iterable,
        urns::{
            ITERABLE_CODER_URN, KV_CODER_URN, NULLABLE_CODER_URN, STR_UTF8_CODER_URN,
            UNIT_CODER_URN, VARINT_CODER_URN,
        },
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
pub trait ElemType: AsAny + Send + Sync + 'static {
    fn default_coder_urn() -> &'static str
    where
        Self: Sized;
}

impl<E: ElemType> ElemType for Vec<E> {
    fn default_coder_urn() -> &'static str {
        ITERABLE_CODER_URN // TODO: BYTES_CODER_URN for Vec<u8>
    }
}

impl ElemType for Bytes {}

impl<K, V> ElemType for KV<K, V>
where
    K: Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default_coder_urn() -> &'static str {
        KV_CODER_URN
    }
}

impl<E: ElemType + fmt::Debug> ElemType for Iterable<E> {
    fn default_coder_urn() -> &'static str {
        ITERABLE_CODER_URN
    }
}

impl ElemType for String {
    fn default_coder_urn() -> &'static str {
        STR_UTF8_CODER_URN
    }
}

impl ElemType for i8 {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for i16 {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for i32 {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for i64 {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for isize {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for u8 {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for u16 {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for u32 {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for u64 {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}
impl ElemType for usize {
    fn default_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }
}

impl ElemType for () {
    fn default_coder_urn() -> &'static str {
        UNIT_CODER_URN
    }
}

impl<E1: ElemType, E2: ElemType> ElemType for (E1, E2) {
    fn default_coder_urn() -> &'static str {
        ITERABLE_CODER_URN
    }
}
impl<E1: ElemType, E2: ElemType, E3: ElemType> ElemType for (E1, E2, E3) {
    fn default_coder_urn() -> &'static str {
        ITERABLE_CODER_URN
    }
}
impl<E1: ElemType, E2: ElemType, E3: ElemType, E4: ElemType> ElemType for (E1, E2, E3, E4) {
    fn default_coder_urn() -> &'static str {
        ITERABLE_CODER_URN
    }
}
impl<E1: ElemType, E2: ElemType, E3: ElemType, E4: ElemType, E5: ElemType> ElemType
    for (E1, E2, E3, E4, E5)
{
    fn default_coder_urn() -> &'static str {
        ITERABLE_CODER_URN
    }
}

impl<E: ElemType> ElemType for Option<E> {
    fn default_coder_urn() -> &'static str {
        NULLABLE_CODER_URN
    }
}
