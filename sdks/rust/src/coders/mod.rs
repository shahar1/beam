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
/// let coder = StrUtf8Coder::default();
///
/// let mut w1 = vec![].writer();
/// coder
///     .encode("my string".to_string(), &mut w1, &Context::WholeStream)
///     .unwrap();
/// w1.flush().unwrap();
/// println!("{:?}", w1.into_inner()); // <= Prints the pure byte-encoding of the string
///
/// let mut w2 = vec![].writer();
/// coder
///     .encode("my string".to_string(), &mut w2, &Context::NeedsDelimiters)
///     .unwrap();
/// w2.flush().unwrap();
/// println!("{:?}", w2.into_inner()); // <= Prints a length-prefix string of bytes
/// ```
pub trait Coder: fmt::Debug + Default {
    /// The type of the elements to be encoded/decoded.
    type E;

    fn get_coder_urn() -> &'static str
    where
        Self: Sized;

    /// Encode an element into a stream of bytes
    ///
    /// # Arguments
    ///
    /// - `element` - an element within a PCollection
    /// - `writer` - a writer that interfaces the coder with the output byte stream
    /// - `context` - the context within which the element should be encoded
    fn encode(
        &self,
        element: Self::E,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error>;

    /// Decode an element from an incoming stream of bytes
    ///
    /// # Arguments
    ///
    /// - `reader` - a reader that interfaces the coder with the input byte stream
    /// - `context` - the context within which the element should be encoded
    fn decode(&self, reader: &mut dyn Read, context: &Context) -> Result<Self::E, io::Error>;
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
