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

mod pipeline_construction;
pub use pipeline_construction::CoderForPipeline;
pub(crate) use pipeline_construction::CoderUrnTree;

mod register_coders;
pub(crate) use register_coders::{DecodeFromUrnFn, EncodeFromUrnFn};

use crate::elem_types::ElemType;
use std::fmt;
use std::io::{self, Read, Write};

/// For custom coders, register_coders!() macro implements this trait for you.
pub trait CoderUrn {
    const URN: &'static str;
}

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
///
/// # Coders' lifecycle
///
/// Coders, including their associated element types, are either defined in the SDK (here we call them preset coders) or SDK users' code (custom coders).
/// They are serialized in proto and sent to the runner, and then to the SDK harness.
/// Then the SDK harness deserializes a proto coder and instantiate it.
///
/// In Rust SDK, only coder IDs and URNs are serialized for both preset coders and custom coders,
/// while in some other SDKs, encode/decode functions in custom coders are also serialized.
///
/// The following is the lifecycle of coders in Rust SDK.
///
/// ## 1. Write code
///
/// 1. An SDK user may define custom coders and write call to `register_coders!(CustomCoder1, ...)` in their code.
///
/// ## 2. Build & Deploy
///
/// 1. The SDK user build the code for both their launcher and SDK workers, each of these may have different architectures (M1 Mac launcher and x86-64 Linux workers, for example).
/// 2. The binary for the workers are deployed to the workers and the SDK harnesses start to run.
///
/// ## 3. Start SDK harness
///
/// 1. Before the main function in the worker code, functions to lookup custom coders' types from coder URNs are generated by `register_coders!` macro call.
///
/// ## 4. Construct pipeline & Register coders
///
/// 1. The coder is instantiated by an SDK user's code on pipeline construction time, along with its coder ID.
/// 2. The coder's URN and its ID are serialized.
/// 3. The serialized coder and its ID are sent to the SDK harness via Runner API -> Fn API.
///
/// ## 5. SDK harness gets ready
///
/// 1. The SDK harness receives the serialized coder's URN and its ID from Fn API.
/// 2. The SDK harness deserializes the coder's URN and creates an instance of the coder specified by the URN.
pub trait Coder: fmt::Debug + Default {
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

#[cfg(test)]
mod tests {
    use crate::coders::standard_coders::StrUtf8Coder;

    use super::*;

    #[test]
    fn test_coder_trait_object() {
        let _coder: Box<dyn Coder> = Box::new(StrUtf8Coder::default());
    }
}
