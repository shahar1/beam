use std::{fmt, marker::PhantomData};

use crate::{
    coders::{
        coders::CoderI,
        required_coders::{BytesCoder, Iterable, IterableCoder, KVCoder, KV},
    },
    elem_types::ElemType,
};

/// Resolve a coder (implementing `CoderI) from a coder URN and an `ElemType`.
///
/// You may use original coders by implementing the `CoderResolver` trait.
pub trait CoderResolver {
    type E: ElemType;
    type C: CoderI<E = Self::E>;

    /// Resolve a coder from a coder URN.
    ///
    /// # Returns
    ///
    /// `Some(C)` if the coder was resolved, `None` otherwise.
    fn resolve(coder_urn: &str) -> Option<Self::C> {
        (coder_urn == Self::C::get_coder_urn()).then_some(Self::C::default())
    }
}

/// `Vec<u8>` -> `BytesCoder`.
#[derive(Debug)]
pub struct BytesCoderResolverDefault;

impl CoderResolver for BytesCoderResolverDefault {
    type E = Vec<u8>;
    type C = BytesCoder;
}

/// `KV` -> `KVCoder`.
#[derive(Debug)]
pub struct KVCoderResolverDefault<K, V> {
    phantom: PhantomData<KV<K, V>>,
}

impl<K, V> CoderResolver for KVCoderResolverDefault<K, V>
where
    K: fmt::Debug + Send + 'static,
    V: fmt::Debug + Send + 'static,
{
    type E = KV<K, V>;
    type C = KVCoder<Self::E>;
}

/// `Iterable` -> `IterableCoder`.
#[derive(Debug)]
pub struct IterableCoderResolverDefault<E>
where
    E: ElemType,
{
    phantom: PhantomData<E>,
}

impl<E> CoderResolver for IterableCoderResolverDefault<E>
where
    E: ElemType + fmt::Debug,
{
    type E = Iterable<E>;
    type C = IterableCoder<Self::E>;
}
