use std::{fmt, marker::PhantomData};

use integer_encoding::VarInt;

use crate::{
    coders::{
        coders::CoderI,
        required_coders::{BytesCoder, Iterable, IterableCoder, KVCoder, KV},
        standard_coders::{StrUtf8Coder, VarIntCoder},
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
    K: Clone + fmt::Debug + Send + 'static,
    V: Clone + fmt::Debug + Send + 'static,
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

/// `String` -> `StrUtf8Coder`.
#[derive(Debug)]
pub struct StrUtf8CoderResolverDefault;

impl CoderResolver for StrUtf8CoderResolverDefault {
    type E = String;
    type C = StrUtf8Coder;
}

#[derive(Debug)]
pub struct VarIntCoderResolverDefault<N: fmt::Debug + VarInt> {
    phantom: PhantomData<N>,
}

impl CoderResolver for VarIntCoderResolverDefault<i8> {
    type E = i8;
    type C = VarIntCoder<i8>;
}
impl CoderResolver for VarIntCoderResolverDefault<i16> {
    type E = i16;
    type C = VarIntCoder<i16>;
}
impl CoderResolver for VarIntCoderResolverDefault<i32> {
    type E = i32;
    type C = VarIntCoder<i32>;
}
impl CoderResolver for VarIntCoderResolverDefault<i64> {
    type E = i64;
    type C = VarIntCoder<i64>;
}
impl CoderResolver for VarIntCoderResolverDefault<isize> {
    type E = isize;
    type C = VarIntCoder<isize>;
}
impl CoderResolver for VarIntCoderResolverDefault<u8> {
    type E = u8;
    type C = VarIntCoder<u8>;
}
impl CoderResolver for VarIntCoderResolverDefault<u16> {
    type E = u16;
    type C = VarIntCoder<u16>;
}
impl CoderResolver for VarIntCoderResolverDefault<u32> {
    type E = u32;
    type C = VarIntCoder<u32>;
}
impl CoderResolver for VarIntCoderResolverDefault<u64> {
    type E = u64;
    type C = VarIntCoder<u64>;
}
impl CoderResolver for VarIntCoderResolverDefault<usize> {
    type E = usize;
    type C = VarIntCoder<usize>;
}
