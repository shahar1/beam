/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::marker::PhantomData;
use std::{any::Any, collections::HashMap, fmt};

pub const BYTES_CODER_URN: &str = "beam:coder:bytes:v1";
pub const KV_CODER_URN: &str = "beam:coder:kvcoder:v1";
pub const ITERABLE_CODER_URN: &str = "beam:coder:iterable:v1";

pub struct CoderRegistry {
    internal_registry: HashMap<&'static str, CoderTypeDiscriminants>,
}

impl CoderRegistry {
    pub fn new() -> Self {
        let internal_registry: HashMap<&'static str, CoderTypeDiscriminants> = HashMap::from([
            (BYTES_CODER_URN, CoderTypeDiscriminants::Bytes),
            (KV_CODER_URN, CoderTypeDiscriminants::KV),
            (ITERABLE_CODER_URN, CoderTypeDiscriminants::Iterable),
        ]);

        Self { internal_registry }
    }

    pub fn get_coder_type(&self, urn: &str) -> &CoderTypeDiscriminants {
        let coder_type = self
            .internal_registry
            .get(urn)
            .unwrap_or_else(|| panic!("No coder type registered for URN {urn}"));

        coder_type
    }

    pub fn register(&mut self, urn: &'static str, coder_type: CoderTypeDiscriminants) {
        self.internal_registry.insert(urn, coder_type);
    }
}

impl Default for CoderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, EnumDiscriminants)]
pub enum CoderType {
    Bytes,
    Iterable,
    KV,
    Placeholder,
}

// TODO: create and use separate AnyCoder trait instead of Any
// ...

/// This is the base interface for coders, which are responsible in Apache Beam to encode and decode
///  elements of a PCollection.
pub trait CoderI<T> {
    fn get_coder_type(&self) -> &CoderTypeDiscriminants;

    fn decode(&self, bytes: Vec<u8>) -> T;

    fn encode(&self, element: T) -> Vec<u8>;

    // TODO: only used temporarily for coder testing, should be moved elsewhere
    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> T;
}

#[derive(Clone)]
pub struct BytesCoder {
    coder_type: CoderTypeDiscriminants,
    urn: &'static str,
}

impl BytesCoder {
    pub fn new() -> Self {
        BytesCoder {
            coder_type: CoderTypeDiscriminants::Bytes,
            urn: BYTES_CODER_URN,
        }
    }
}

impl CoderI<Vec<u8>> for BytesCoder {
    fn get_coder_type(&self) -> &CoderTypeDiscriminants {
        &self.coder_type
    }

    fn encode(&self, element: Vec<u8>) -> Vec<u8> {
        element
    }

    fn decode(&self, bytes: Vec<u8>) -> Vec<u8> {
        bytes
    }

    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> Vec<u8> {
        value.as_str().unwrap().as_bytes().to_vec()
    }
}

impl Default for BytesCoder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BytesCoder {
    fn fmt<'a>(&'a self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_struct("BytesCoder")
            .field("urn", &self.urn)
            .finish()
    }
}

#[derive(Clone)]
pub struct KV<K, V> {
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K, V> KV<K, V> {
    pub fn new() -> Self {
        KV {
            k: PhantomData::default(),
            v: PhantomData::default(),
        }
    }
}

#[derive(Clone)]
pub struct KVCoder<KV> {
    coder_type: CoderTypeDiscriminants,
    urn: &'static str,

    phantom: PhantomData<KV>,
}

impl<K, V> CoderI<KV<K, V>> for KVCoder<KV<K, V>> {
    fn get_coder_type(&self) -> &CoderTypeDiscriminants {
        &self.coder_type
    }

    fn encode(&self, element: KV<K, V>) -> Vec<u8> {
        Vec::new()
    }

    fn decode(&self, bytes: Vec<u8>) -> KV<K, V> {
        KV::new()
    }

    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> KV<K, V> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct IterableCoder<T> {
    coder_type: CoderTypeDiscriminants,
    urn: &'static str,

    phantom: PhantomData<T>,
}

pub struct Iterable<T> {
    phantom: PhantomData<T>,
}

impl<T> CoderI<Iterable<T>> for IterableCoder<T> {
    fn get_coder_type(&self) -> &CoderTypeDiscriminants {
        &self.coder_type
    }

    fn encode(&self, element: Iterable<T>) -> Vec<u8> {
        Vec::new()
    }

    fn decode(&self, bytes: Vec<u8>) -> Iterable<T> {
        Iterable {
            phantom: PhantomData,
        }
    }

    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> Iterable<T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    use serde::Deserialize;
    use serde_yaml::{Deserializer, Value};

    // TODO: empty this list
    const UNSUPPORTED_CODERS: [&'static str; 15] = [
        "beam:coder:bool:v1",
        "beam:coder:string_utf8:v1",
        "beam:coder:varint:v1",
        "beam:coder:kv:v1",
        "beam:coder:interval_window:v1",
        "beam:coder:iterable:v1",
        "beam:coder:state_backed_iterable:v1",
        "beam:coder:timer:v1",
        "beam:coder:global_window:v1",
        "beam:coder:windowed_value:v1",
        "beam:coder:param_windowed_value:v1",
        "beam:coder:double:v1",
        "beam:coder:row:v1",
        "beam:coder:sharded_key:v1",
        "beam:coder:custom_window:v1",
    ];

    const STANDARD_CODERS_FILE: &str =
        "model/fn-execution/src/main/resources/org/apache/beam/model/fnexecution/v1/standard_coders.yaml";

    #[test]
    fn test_coders() {
        let coder_registry = CoderRegistry::new();

        // TODO: Move this to utils module
        let current_dir = std::env::current_dir().unwrap();
        let beam_root_dir = current_dir
            .as_path()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let standard_coders_dir = beam_root_dir.join(STANDARD_CODERS_FILE);

        let f = std::fs::File::open(standard_coders_dir).expect("Unable to read file");
        for doc in Deserializer::from_reader(f) {
            let spec = Value::deserialize(doc).expect("Unable to parse document");

            let urn_spec = spec
                .get("coder")
                .cloned()
                .unwrap()
                .get("urn")
                .cloned()
                .unwrap();
            let urn = urn_spec.as_str().unwrap();

            if UNSUPPORTED_CODERS.contains(&urn) {
                continue;
            }

            let nested_field = spec.get("nested");
            if nested_field.is_some() && nested_field.unwrap().as_bool().unwrap() {
                // TODO: support nesting of coders
                continue;
            }

            let coder_type = coder_registry.get_coder_type(urn);
            let nested = false;

            // TODO: generalize
            match coder_type {
                CoderTypeDiscriminants::Bytes => {
                    let c = BytesCoder::new();
                    run_unnested::<BytesCoder, Vec<u8>>(&c, nested, &spec);
                }
                _ => unimplemented!(),
            }
        }
    }

    fn run_unnested<'a, C, E>(coder: &(dyn Any + 'a), _nested: bool, spec: &Value)
    where
        C: CoderI<E> + 'a,
        E: Clone + std::fmt::Debug + PartialEq,
    {
        let examples = spec.get("examples").unwrap().as_mapping().unwrap();

        for (expected, original) in examples.iter() {
            run_case::<C, E>(coder, expected, original);
        }
    }

    fn run_case<'a, C, E>(coder: &(dyn Any + 'a), expected_encoded: &Value, original: &Value)
    where
        C: CoderI<E> + 'a,
        E: Clone + std::fmt::Debug + PartialEq,
    {
        let c: &C = coder.downcast_ref::<C>().unwrap();

        let expected_enc = expected_encoded.as_str().unwrap().as_bytes().to_vec();

        // TODO: generalize
        let res = match c.get_coder_type() {
            &CoderTypeDiscriminants::Bytes => {
                let expected_dec = c.parse_yaml_value(original);
                let decoded = c.decode(expected_enc.clone());
                let encoded = c.encode(expected_dec.clone());
                (encoded, decoded, expected_dec)
            }
            _ => unimplemented!(),
        };

        let (encoded, decoded, expected_dec) = res;

        println!("\n---------\nCoder type: {:?}", c.get_coder_type());
        println!(
            "\nExpected encoded: {:?}\nGenerated encoded: {:?}\n\nExpected decoded: {:?}\nGenerated decoded: {:?}",
            encoded, decoded, expected_dec, expected_dec
        );

        assert_eq!(encoded, expected_enc);
        assert_eq!(decoded, expected_dec);
    }
}
