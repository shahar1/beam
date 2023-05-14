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

pub mod coder_resolver;
pub mod required_coders;
pub mod rust_coders;
pub mod standard_coders;
pub mod urns;

use once_cell::sync::OnceCell;

use crate::elem_types::ElemType;
use crate::proto::beam_api::pipeline as proto_pipeline;
use std::fmt;
use std::io::{self, Read, Write};

/// This is the base interface for coders, which are responsible in Apache Beam to encode and decode
/// elements of a PCollection.
///
/// # Example
///
/// ```
/// use apache_beam::coders::{Coder, standard_coders::StrUtf8Coder, Context};
/// use bytes::buf::BufMut;
/// use std::io::Write;
///
/// let element = "my string".to_string();
/// let coder = StrUtf8Coder::default();
///
/// let mut w1 = vec![].writer();
/// coder
///     .encode(&element, &mut w1, &Context::WholeStream)
///     .unwrap();
/// w1.flush().unwrap();
/// println!("{:?}", w1.into_inner()); // <= Prints the pure byte-encoding of the string
///
/// let mut w2 = vec![].writer();
/// coder
///     .encode(&element, &mut w2, &Context::NeedsDelimiters)
///     .unwrap();
/// w2.flush().unwrap();
/// println!("{:?}", w2.into_inner()); // <= Prints a length-prefix string of bytes
/// ```
pub trait Coder: fmt::Debug + Default {
    const URN: &'static str;

    /// Encode an element into a stream of bytes
    ///
    /// # Arguments
    ///
    /// - `element` - an element within a PCollection
    /// - `writer` - a writer that interfaces the coder with the output byte stream
    /// - `context` - the context within which the element should be encoded
    fn encode(
        &self,
        element: &dyn ElemType,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error>;

    /// Decode an element from an incoming stream of bytes
    ///
    /// # Arguments
    ///
    /// - `reader` - a reader that interfaces the coder with the input byte stream
    /// - `context` - the context within which the element should be encoded
    fn decode(
        &self,
        reader: &mut dyn Read,
        context: &Context,
    ) -> Result<Box<dyn ElemType>, io::Error>;

    /// Convert this coder into its protocol buffer representation for the Runner API.
    /// A coder in protobuf format can be shared with other components such as Beam runners,
    /// SDK workers; and reconstructed into its runtime representation if necessary.
    #[doc(hidden)]
    fn to_proto(&self, component_coder_ids: Vec<String>) -> proto_pipeline::Coder {
        let spec = proto_pipeline::FunctionSpec {
            urn: Self::URN.to_string(),
            payload: vec![], // unused in Rust SDK
        };
        proto_pipeline::Coder {
            spec: Some(spec),
            component_coder_ids,
        }
    }
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

/// Must be called in outside of the main() function.
/// 
/// TODO example
#[macro_export]
macro_rules! register_coders {
    ($($coder:ident),*) => {
        fn encode_from_urn(urn: &str, elem: &dyn $crate::elem_types::ElemType, writer: &mut dyn std::io::Write, context: &$crate::coders::Context) -> Result<usize, std::io::Error> {
            match urn {
                $($coder::URN => $coder.encode(elem, writer, context),)*
                _ => panic!("unknown urn"),
            }
        }

        fn decode_from_urn(urn: &str, reader: &mut dyn std::io::Read, context: &$crate::coders::Context) -> Result<Box<dyn $crate::elem_types::ElemType>, std::io::Error> {
            match urn {
                $($coder::URN => $coder.decode(reader, context),)*
                _ => panic!("unknown urn"),
            }
        }

        #[ctor::ctor]
        fn init_coders_from_urn() {
            $crate::coders::CODERS_FROM_URN.set($crate::coders::CodersFromUrn {
                enc: encode_from_urn,
                dec: decode_from_urn,
            }).unwrap();
        }
    }
}

type EncodeFromUrnFn = fn(
    &str,
    &dyn crate::elem_types::ElemType,
    &mut dyn std::io::Write,
    &crate::coders::Context,
) -> Result<usize, std::io::Error>;

type DecodeFromUrnFn = fn(
    &str,
    &mut dyn std::io::Read,
    &crate::coders::Context,
) -> Result<Box<dyn crate::elem_types::ElemType>, std::io::Error>;

pub struct CodersFromUrn {
    pub enc: EncodeFromUrnFn,
    pub dec: DecodeFromUrnFn,
}

impl CodersFromUrn {
    pub fn global() -> &'static CodersFromUrn {
        crate::coders::CODERS_FROM_URN.get().unwrap()
    }

    pub fn encode_from_urn(
        &self,
        urn: &str,
        elem: &dyn crate::elem_types::ElemType,
        writer: &mut dyn std::io::Write,
        context: &crate::coders::Context,
    ) -> Result<usize, std::io::Error> {
        (self.enc)(urn, elem, writer, context)
    }

    pub fn decode_from_urn(
        &self,
        urn: &str,
        reader: &mut dyn std::io::Read,
        context: &crate::coders::Context,
    ) -> Result<Box<dyn crate::elem_types::ElemType>, std::io::Error> {
        (self.dec)(urn, reader, context)
    }
}

impl fmt::Debug for CodersFromUrn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodersFromUrn").finish()
    }
}

pub static CODERS_FROM_URN: OnceCell<CodersFromUrn> = OnceCell::new();
