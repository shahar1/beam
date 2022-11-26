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

use std::any::Any;
use std::collections::HashMap;

static BYTES_CODER_URN: &str = "beam:coder:bytes:v1";
static KV_CODER_URN: &str = "beam:coder:kvcoder:v1";
static ITERABLE_CODER_URN: &str = "beam:coder:iterable:v1";

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

    pub fn get(&self, urn: &'static str) -> Box<dyn Coder> {
        let coder = match self.internal_registry.get(urn).unwrap() {
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
    fn encode(&self, element: Box<dyn Any>) -> &'static [u8] {
        unimplemented!()
    }

    fn decode(&self, bytes: &'static [u8]) -> Box<dyn Any> {
        unimplemented!()
    }
}

pub struct BytesCoder {
    urn: &'static str,
}

impl BytesCoder {
    pub fn new() -> Self {
        BytesCoder {
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
    fn encode(&self, element: Box<dyn Any>) -> &'static [u8] {
        unimplemented!()
    }

    fn decode(&self, bytes: &'static [u8]) -> Box<dyn Any> {
        unimplemented!()
    }
}

pub struct KVCoder {
    urn: &'static str,
    key_coder_type: CoderType,
    value_coder_type: CoderType,
}

impl KVCoder {
    pub fn new() -> Self {
        KVCoder {
            urn: KV_CODER_URN,
            key_coder_type: CoderType::PlaceholderCoder,
            value_coder_type: CoderType::PlaceholderCoder,
        }
    }
}

impl Default for KVCoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Coder for KVCoder {
    fn encode(&self, element: Box<dyn Any>) -> &'static [u8] {
        unimplemented!()
    }

    fn decode(&self, bytes: &'static [u8]) -> Box<dyn Any> {
        unimplemented!()
    }
}

pub struct IterableCoder {
    urn: &'static str,
    element_coder_type: CoderType,
}

impl IterableCoder {
    pub fn new() -> Self {
        IterableCoder {
            urn: ITERABLE_CODER_URN,
            element_coder_type: CoderType::PlaceholderCoder,
        }
    }
}

impl Default for IterableCoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Coder for IterableCoder {
    fn encode(&self, element: Box<dyn Any>) -> &'static [u8] {
        unimplemented!()
    }

    fn decode(&self, bytes: &'static [u8]) -> Box<dyn Any> {
        unimplemented!()
    }
}
