use std::marker::PhantomData;

use crate::{
    coders::{
        coders::CoderI,
        required_coders::{BytesCoder, KVCoder, KV},
    },
    elem_types::ElemType,
};

/// Resolve a coder (implementing `CoderI) from a coder URN and an `ElemType`.
///
/// You may use original coders by implementing the `CoderResolver` trait.
pub trait CoderResolver {
    type E: ElemType;

    /// Resolve a coder from a coder URN.
    ///
    /// # Returns
    ///
    /// `Some(C)` if the coder was resolved, `None` otherwise.
    fn resolve(&self, coder_urn: &str) -> Option<Box<dyn CoderI<E = Self::E>>>;

    fn _resolve_default<C: CoderI<E = Self::E> + Default + 'static>(
        &self,
        coder_urn: &str,
    ) -> Option<Box<dyn CoderI<E = Self::E>>> {
        (coder_urn == C::get_coder_urn()).then_some(Box::<C>::default())
    }
}

/// `Vec<u8>` -> `BytesCoder`.
#[derive(Debug)]
pub struct BytesCoderResolverDefault;

impl CoderResolver for BytesCoderResolverDefault {
    type E = Vec<u8>;

    fn resolve(&self, coder_urn: &str) -> Option<Box<dyn CoderI<E = Self::E>>> {
        self._resolve_default::<BytesCoder>(coder_urn)
    }
}

/// `KV` -> `KVCoder`.
#[derive(Debug)]
pub struct KVCoderResolverDefault<K, V> {
    phantom: PhantomData<KV<K, V>>,
}

impl<K, V> CoderResolver for KVCoderResolverDefault<K, V>
where
    K: Send + 'static,
    V: Send + 'static,
{
    type E = KV<K, V>;

    fn resolve(&self, coder_urn: &str) -> Option<Box<dyn CoderI<E = Self::E>>> {
        self._resolve_default::<KVCoder<KV<K, V>>>(coder_urn)
    }
}
