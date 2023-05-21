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

//! Defines all of the Apache Beam required coders.
//!
//! These are the coders necessary for encoding the data types required by
//! the Apache Beam model. They provide standardized ways of encode data for
//! communication between the runner, the Beam workers, and the user's code.
//! For example for any aggregations the runner and the SDK need to agree on
//! the encoding of key-value pairs; so that the SDK will encode keys properly,
//! and the runner will be able to group elements of the
//! same key together.
//!
//! The formal specifications for these coders can be found in
//! model/pipeline/src/main/proto/beam_runner_api.proto

use std::fmt;
use std::io::{self, Read, Write};
use std::marker::PhantomData;

use bytes::Bytes;
use integer_encoding::{VarIntReader, VarIntWriter};

use crate::coders::{urns::*, CoderForPipeline, CoderUrn, CoderUrnTree};
use crate::coders::{Coder, Context};
use crate::elem_types::{DefaultCoder, ElemType};

/// Coder for byte-array data types
#[derive(Clone, Default)]
pub struct BytesCoder {}

impl CoderUrn for BytesCoder {
    const URN: &'static str = BYTES_CODER_URN;
}

impl Coder for BytesCoder {
    fn new(_component_coders: Vec<Box<dyn Coder>>) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    /// Encode the input element (a byte-string) into the output byte stream from `writer`.
    /// If context is `NeedsDelimiters`, the byte string is encoded prefixed with a
    /// varint representing its length.
    ///
    /// If the context is `WholeStream`, the byte string is encoded as-is.
    fn encode(
        &self,
        element: &dyn ElemType,
        mut writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error> {
        let element = element.as_any().downcast_ref::<Bytes>().unwrap();

        match context {
            Context::WholeStream => writer.write(element),
            Context::NeedsDelimiters => {
                // TODO: confirm that usize gets decoded correctly by production runners
                let delimiter: usize = element.len();
                writer
                    .write_varint(delimiter)
                    .expect("Unable to write delimiter to buffer");

                writer.write(element)
            }
        }
    }

    /// Decode the input byte stream into a byte array.
    /// If context is `NeedsDelimiters`, the first bytes will be interpreted as a var-int32 encoding
    /// the length of the data.
    ///
    /// If the context is `WholeStream`, the whole input stream is decoded as-is.
    fn decode(
        &self,
        mut reader: &mut dyn Read,
        context: &Context,
    ) -> Result<Box<dyn ElemType>, io::Error> {
        match context {
            Context::WholeStream => {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                Ok(Box::new(Bytes::from(buf)))
            }

            Context::NeedsDelimiters => {
                let delimiter: usize = reader
                    .read_varint()
                    .expect("Unable to read delimiter from buffer");

                let mut buf: Vec<u8> = Vec::new();

                let mut num_bytes_read = 0;
                for b in reader.bytes() {
                    if num_bytes_read == delimiter {
                        break;
                    }

                    if b.is_err() {
                        return Err(io::Error::from(std::io::ErrorKind::UnexpectedEof));
                    }

                    buf.push(b.expect("Unable to read byte"));
                    num_bytes_read += 1;
                }

                if num_bytes_read < delimiter {
                    return Err(io::Error::from(std::io::ErrorKind::UnexpectedEof));
                }

                Ok(Box::new(Bytes::from(buf)))
            }
        }
    }
}

impl CoderForPipeline for BytesCoder {
    fn component_coder_urns() -> Vec<CoderUrnTree> {
        vec![]
    }
}

impl fmt::Debug for BytesCoder {
    fn fmt(&self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_struct("BytesCoder")
            .field("urn", &Self::URN)
            .finish()
    }
}

/// A coder for a key-value pair
pub struct KVCoder<K, V> {
    _key_coder: Box<dyn Coder>,
    _value_coder: Box<dyn Coder>,

    phantom: PhantomData<(K, V)>,
}

impl<K, V> CoderUrn for KVCoder<K, V>
where
    K: ElemType,
    V: ElemType,
{
    const URN: &'static str = KV_CODER_URN;
}

impl Coder for KVCoder<(), ()> {
    fn new(mut component_coders: Vec<Box<dyn Coder>>) -> Self
    where
        Self: Sized,
    {
        let value_coder = component_coders
            .pop()
            .expect("2nd component coder should be value coder");
        let key_coder = component_coders
            .pop()
            .expect("1st component coder should be key coder");

        Self {
            _key_coder: key_coder,
            _value_coder: value_coder,
            phantom: PhantomData,
        }
    }

    /// Encode the input element (a key-value pair) into a byte output stream. They key and value are encoded one after the
    /// other (first key, then value). The key is encoded with `Context::NeedsDelimiters`, while the value is encoded with
    /// the input context of the `KVCoder`.
    fn encode(
        &self,
        _element: &dyn ElemType,
        _writer: &mut dyn Write,
        _context: &Context,
    ) -> Result<usize, io::Error> {
        todo!()
    }

    /// Decode the input byte stream into a `KV` element
    fn decode(
        &self,
        _reader: &mut dyn Read,
        _context: &Context,
    ) -> Result<Box<dyn ElemType>, io::Error> {
        todo!()
    }
}

impl<K, V> CoderForPipeline for KVCoder<K, V>
where
    K: ElemType + DefaultCoder,
    V: ElemType + DefaultCoder,
{
    fn component_coder_urns() -> Vec<CoderUrnTree> {
        vec![K::default_coder_urn(), V::default_coder_urn()]
    }
}

impl<K, V> fmt::Debug for KVCoder<K, V>
where
    K: ElemType,
    V: ElemType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KVCoder").finish()
    }
}

/// A coder for a 'list' or a series of elements of the same type
#[derive(Debug)]
pub struct IterableCoder<E>
where
    E: ElemType,
{
    _elem_coder: Box<dyn Coder>,

    phantom: PhantomData<E>,
}

#[derive(Clone, Debug)]
pub struct Iterable<E>
where
    E: ElemType,
{
    phantom: PhantomData<E>,
}

impl<ItE> CoderUrn for IterableCoder<ItE>
where
    ItE: ElemType + fmt::Debug,
{
    const URN: &'static str = ITERABLE_CODER_URN;
}

impl Coder for IterableCoder<()> {
    fn new(mut component_coders: Vec<Box<dyn Coder>>) -> Self
    where
        Self: Sized,
    {
        let elem_coder = component_coders
            .pop()
            .expect("1st component coder should be element coder");
        Self {
            _elem_coder: elem_coder,
            phantom: PhantomData,
        }
    }

    /// Encode the input iterable into a byte output stream. Elements can be encoded in two different ways:
    ///
    /// - If the length of the input iterable is known a-priori, then the length is encoded with a 32-bit
    ///     fixed-length integer.
    /// - If the length of the input iterable is not known a-priori, then a 32-bit integer with a value
    ///     of `-1` is encoded in the first position (instead of the length), and
    ///
    /// Then, each element is encoded individually in `Context::NeedsDelimiters`.
    fn encode(
        &self,
        _element: &dyn ElemType,
        _writer: &mut dyn Write,
        _context: &Context,
    ) -> Result<usize, io::Error> {
        todo!()
    }

    /// Decode the input byte stream into a `Iterable` element
    fn decode(
        &self,
        _reader: &mut dyn Read,
        _context: &Context,
    ) -> Result<Box<dyn ElemType>, io::Error> {
        todo!()
    }
}

impl<ItE> CoderForPipeline for IterableCoder<ItE>
where
    ItE: ElemType + DefaultCoder + fmt::Debug,
{
    fn component_coder_urns() -> Vec<CoderUrnTree> {
        vec![ItE::default_coder_urn()]
    }
}
