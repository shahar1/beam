use std::cmp;

use crate::elem_types::ElemType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KV<K, V> {
    pub k: K,
    pub v: V,
}

impl<K, V> KV<K, V>
where
    K: ElemType,
    V: ElemType,
{
    pub fn new(k: K, v: V) -> Self {
        KV { k, v }
    }

    pub fn as_values(&self) -> &V {
        &self.v
    }
}

impl<K, V> PartialOrd for KV<K, V>
where
    K: PartialOrd,
    V: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.k.partial_cmp(&other.k) {
            Some(cmp::Ordering::Equal) => self.v.partial_cmp(&other.v),
            els => els,
        }
    }
}

impl<K, V> Ord for KV<K, V>
where
    K: Ord,
    V: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.k.cmp(&other.k) {
            cmp::Ordering::Equal => self.v.cmp(&other.v),
            els => els,
        }
    }
}
