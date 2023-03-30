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

pub mod required_coders;
pub mod rust_coders;
pub mod standard_coders;
pub mod urns;

use std::collections::HashMap;
use std::io::{self, Read, Write};

use crate::coders::urns::*;

pub struct CoderRegistry {
    internal_registry: HashMap<&'static str, CoderTypeDiscriminants>,
}

impl CoderRegistry {
    pub fn new() -> Self {
        let internal_registry: HashMap<&'static str, CoderTypeDiscriminants> = HashMap::from([
            (BYTES_CODER_URN, CoderTypeDiscriminants::Bytes),
            (
                GENERAL_OBJECT_CODER_URN,
                CoderTypeDiscriminants::GeneralObject,
            ),
            (KV_CODER_URN, CoderTypeDiscriminants::KV),
            (ITERABLE_CODER_URN, CoderTypeDiscriminants::Iterable),
            (STR_UTF8_CODER_URN, CoderTypeDiscriminants::StrUtf8),
            (VARINT_CODER_URN, CoderTypeDiscriminants::VarIntCoder),
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
    // ******* Required coders *******
    Bytes,
    Iterable,
    KV,

    // ******* Rust coders *******
    GeneralObject,

    // ******* Standard coders *******
    StrUtf8,
    VarIntCoder,
}

// TODO: create and use separate AnyCoder trait instead of Any
// ...

/// This is the base interface for coders, which are responsible in Apache Beam to encode and decode
///  elements of a PCollection.
pub trait CoderI<T> {
    fn get_coder_type(&self) -> &CoderTypeDiscriminants;

    /// Encode an element into a stream of bytes
    fn encode(
        &self,
        element: T,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error>;

    /// Decode an element from an incoming stream of bytes
    fn decode(&self, reader: &mut dyn Read, context: &Context) -> Result<T, io::Error>;
}

/// The context for encoding a PCollection element.
/// For example, for strings of utf8 characters or bytes, `WholeStream` encoding means
/// that the string will be encoded as-is; while `NeedsDelimiter` encoding means that the
/// string will be encoded prefixed with its length.
pub enum Context {
    /// Whole stream encoding/decoding means that the encoding/decoding function does not need to worry about
    /// delimiting the start and end of the current element in the stream of bytes.
    WholeStream,

    /// Needs-delimiters encoding means that the encoding of data must be such that when decoding,
    /// the coder is able to stop decoding data at the end of the current element.
    NeedsDelimiters,
}
