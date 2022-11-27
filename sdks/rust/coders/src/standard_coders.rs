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

use std::collections::HashMap;

const BYTES_CODER_URN: &str = "beam:coder:bytes:v1";
const KV_CODER_URN: &str = "beam:coder:kvcoder:v1";
const ITERABLE_CODER_URN: &str = "beam:coder:iterable:v1";

#[derive(Clone)]
pub enum CoderType {
    BytesCoder,
    KVCoder,
    IterableCoder,
    PlaceholderCoder,
}

pub struct CoderRegistry {
    internal_registry: HashMap<&'static str, CoderType>,
}

impl CoderRegistry {
    pub fn new() -> Self {
        let internal_registry: HashMap<&'static str, CoderType> = HashMap::from([
            (BYTES_CODER_URN, CoderType::BytesCoder),
            (KV_CODER_URN, CoderType::KVCoder),
            (ITERABLE_CODER_URN, CoderType::IterableCoder),
        ]);

        Self { internal_registry }
    }

    pub fn get_coder(&self, urn: &str) -> Box<dyn Coder> {
        let coder_type = self
            .internal_registry
            .get(urn)
            .unwrap_or_else(|| panic!("No coder type registered for URN {urn}"));
        let coder = match coder_type {
            CoderType::BytesCoder => BytesCoder::new(),
            _ => unimplemented!(),
        };

        Box::new(coder)
    }

    pub fn register(&mut self, urn: &'static str, coder_type: CoderType) {
        self.internal_registry.insert(urn, coder_type);
    }
}

impl Default for CoderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Coder {
    fn get_coder_type(&self) -> CoderType;

    fn encode_to_bytes<'a>(&'a self, element: &'a [u8]) -> &[u8] {
        panic!("Invalid operation for this type of coder");
    }

    fn decode_to_bytes<'a>(&'a self, bytes: &'a [u8]) -> &[u8] {
        panic!("Invalid operation for this type of coder");
    }
}

pub struct BytesCoder {
    urn: &'static str,
    coder_type: CoderType,
}

impl BytesCoder {
    pub fn new() -> Self {
        BytesCoder {
            coder_type: CoderType::BytesCoder,
            urn: BYTES_CODER_URN,
        }
    }
}

impl Default for BytesCoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Coder for BytesCoder {
    fn get_coder_type(&self) -> CoderType {
        CoderType::BytesCoder
    }

    fn encode_to_bytes<'a>(&'a self, element: &'a [u8]) -> &[u8] {
        element
    }

    fn decode_to_bytes<'a>(&'a self, bytes: &'a [u8]) -> &[u8] {
        bytes
    }
}

// pub struct KVCoder {
//     urn: &'static str,
//     coder_type: CoderType,
//     key_coder_type: CoderType,
//     value_coder_type: CoderType,
// }

// impl KVCoder {
//     pub fn new() -> Self {
//         KVCoder {
//             urn: KV_CODER_URN,
//             key_coder_type: CoderType::PlaceholderCoder,
//             value_coder_type: CoderType::PlaceholderCoder,
//         }
//     }
// }

// impl Default for KVCoder {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Coder for KVCoder {
// }

// pub struct IterableCoder {
//     coder_type: CoderType,
//     element_coder_type: CoderType,
//     urn: &'static str,
// }

// impl IterableCoder {
//     pub fn new() -> Self {
//         IterableCoder {
//             urn: ITERABLE_CODER_URN,
//             element_coder_type: CoderType::PlaceholderCoder,
//         }
//     }
// }

// impl Default for IterableCoder {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Coder for IterableCoder {
// }

#[cfg(test)]
mod tests {
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

            let coder = coder_registry.get_coder(urn);
            let nested = false;
            run_unnested(coder, nested, &spec);
        }
    }

    fn run_unnested(coder: Box<dyn Coder>, _nested: bool, spec: &Value) {
        let examples = spec.get("examples").unwrap().as_mapping().unwrap();

        for (expected, original) in examples.iter() {
            run_case(&coder, expected, original);
        }
    }

    fn run_case<'a>(coder: &Box<dyn Coder>, expected_encoded: &'a Value, original: &'a Value) {
        let res = match coder.get_coder_type() {
            // TODO: refactor
            CoderType::BytesCoder => {
                let obj = original.as_str().unwrap().as_bytes();
                let encoded = coder.encode_to_bytes(obj);
                let decoded = coder.decode_to_bytes(expected_encoded.as_str().unwrap().as_bytes());
                let expected_enc = expected_encoded.as_str().unwrap().as_bytes();
                let expected_dec = expected_encoded.as_str().unwrap().as_bytes();
                (encoded, decoded, expected_enc, expected_dec)
            }
            _ => unimplemented!(),
        };

        let (encoded, decoded, expected_enc, expected_dec) = res;

        assert_eq!(encoded, expected_enc);
        assert_eq!(decoded, expected_dec);
    }
}
