use std::{cmp, fmt};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KV<K, V> {
    k: K,
    v: V,
}

impl<K, V> KV<K, V>
where
    K: Clone + fmt::Debug + Send,
    V: Clone + fmt::Debug + Send,
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
