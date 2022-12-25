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

// TODO: make separate required_coders file

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

use std::io::{self, Read, Write, ErrorKind};
use std::marker::PhantomData;
use std::{collections::HashMap, fmt};

use crate::coders::Context;

// TODO: reorganize modules and separate coders by type
pub const BYTES_CODER_URN: &str = "beam:coder:bytes:v1";
pub const KV_CODER_URN: &str = "beam:coder:kvcoder:v1";
pub const ITERABLE_CODER_URN: &str = "beam:coder:iterable:v1";

pub const STR_UTF8_CODER_URN: &str = "beam:coder:string_utf8:v1";

pub struct CoderRegistry {
    internal_registry: HashMap<&'static str, CoderTypeDiscriminants>,
}

impl CoderRegistry {
    pub fn new() -> Self {
        let internal_registry: HashMap<&'static str, CoderTypeDiscriminants> = HashMap::from([
            (BYTES_CODER_URN, CoderTypeDiscriminants::Bytes),
            (KV_CODER_URN, CoderTypeDiscriminants::KV),
            (ITERABLE_CODER_URN, CoderTypeDiscriminants::Iterable),
            (STR_UTF8_CODER_URN, CoderTypeDiscriminants::StrUtf8),
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
    StrUtf8,

    Placeholder,
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

trait CoderTestUtils
where <Self as CoderTestUtils>::InternalCoderType: Clone +  fmt::Debug
{
    type InternalCoderType;

    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> <Self as CoderTestUtils>::InternalCoderType;
}

#[derive(Clone)]
pub struct StrUtf8Coder {
    coder_type: CoderTypeDiscriminants,
    urn: &'static str,
}

impl StrUtf8Coder {
    pub fn new() -> Self {
        Self {
            coder_type: CoderTypeDiscriminants::StrUtf8,
            urn: STR_UTF8_CODER_URN,
        }
    }
}

// TODO: fix implementation for unicode values and retest this
// TODO: accept string references as well?
impl CoderI<String> for StrUtf8Coder {
    fn get_coder_type(&self) -> &CoderTypeDiscriminants {
        &self.coder_type
    }

    fn encode(
        &self,
        element: String,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error> {
        let bytes = element.as_bytes().to_vec();
        let coder = BytesCoder::new();
        coder.encode(bytes, writer, context)
    }

    fn decode(&self, reader: &mut dyn Read, context: &Context) -> Result<String, io::Error> {
        let coder = BytesCoder::new();
        let bytes = coder.decode(reader, context)?;
        
        let res = String::from_utf8(bytes);
    
        //TODO: improve error handling
        match res {
            Ok(s) => Ok(s),
            Err(_) => {
                Result::Err(io::Error::new(ErrorKind::Other, "Unable to convert bytes to string"))    
            }
        }
    }
}

impl CoderTestUtils for StrUtf8Coder
{
    type InternalCoderType = String;

    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> <StrUtf8Coder as CoderTestUtils>::InternalCoderType {
        value.as_str().unwrap().to_string()
    }
}

impl Default for StrUtf8Coder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for StrUtf8Coder {
    fn fmt<'a>(&'a self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_struct("StrUtf8Coder")
            .field("urn", &self.urn)
            .finish()
    }
}

/// Coder for byte-array data types
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

    /// Encode the input element (a byte-string) into the output byte stream from `writer`.
    /// If context is `NeedsDelimiters`, the byte string is encoded prefixed with a
    /// varint representing its length.
    ///
    /// If the context is `WholeStream`, the byte string is encoded as-is.
    fn encode(
        &self,
        element: Vec<u8>,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error> {
        match context {
            // TODO: write to the buffer ignoring the preceding bytes that would
            // be used as a u32 to represent a delimiter (when that functionality
            // gets implemented)
            Context::WholeStream => writer.write(&element),
            Context::NeedsDelimiters => writer.write(&element),
        }
    }

    /// Decode the input byte stream into a byte array.
    /// If context is `NeedsDelimiters`, the first bytes will be interpreted as a var-int32 encoding
    /// the length of the data.
    ///
    /// If the context is `WholeStream`, the whole input stream is decoded as-is.
    fn decode(&self, reader: &mut dyn Read, context: &Context) -> Result<Vec<u8>, io::Error> {
        match context {
            Context::WholeStream => {
                let mut buf: Vec<u8> = Vec::new();
                reader.read_to_end(&mut buf)?;
                Ok(buf)
            }
            // TODO: interpret the preceding bytes in the buffer as a u32 flag
            // and use that value as a length delimiter (whenever it gets
            // implemented)
            Context::NeedsDelimiters => {
                unimplemented!()

                // // Read initial bytes as u32
                // let length = ...

                // let mut buf: Vec<u8> = Vec::new();

                // // Read as many bytes as possible (or should it fail if there
                // // are fewer bytes available than length?)
                // let mut num_bytes_read = 0;
                // for b in reader.bytes() {
                //     if num_bytes_read == length || b.is_err() {
                //         break;
                //     }

                //     buf.push(b.unwrap());
                //     num_bytes_read += 1;
                // }

                // use std::io::ErrorKind;
                // if num_bytes_read == 0 {
                //     return Err(io::Error::from(ErrorKind::UnexpectedEof));
                // }

                // Ok(buf)
            }
        }
    }

}

impl CoderTestUtils for BytesCoder
{
    type InternalCoderType = Vec<u8>;

    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> <BytesCoder as CoderTestUtils>::InternalCoderType {
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

#[derive(Clone, fmt::Debug)]
pub struct KV<K, V>
{
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K, V> KV<K, V> 
where
    K: Clone + fmt::Debug,
    V: Clone + fmt::Debug
{
    pub fn new() -> Self {
        KV {
            k: PhantomData::default(),
            v: PhantomData::default(),
        }
    }
}

/// A coder for a key-value pair
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

    /// Encode the input element (a key-value pair) into a byte output stream. They key and value are encoded one after the
    /// other (first key, then value). The key is encoded with `Context::NeedsDelimiters`, while the value is encoded with
    /// the input context of the `KVCoder`.
    fn encode(
        &self,
        element: KV<K, V>,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error> {
        unimplemented!()
    }

    /// Decode the input byte stream into a `KV` element
    fn decode(&self, reader: &mut dyn Read, context: &Context) -> Result<KV<K, V>, io::Error> {
        unimplemented!()
    }

}

impl<K, V> CoderTestUtils for KVCoder<KV<K, V>>
where
    K: Clone + fmt::Debug,
    V: Clone + fmt::Debug
{
    type InternalCoderType = KV<K, V>;

    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> <KVCoder<KV<K, V>> as CoderTestUtils>::InternalCoderType {
        unimplemented!()
    }
}

/// A coder for a 'list' or a series of elements of the same type
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
        element: Iterable<T>,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error> {
        unimplemented!()
    }

    /// Decode the input byte stream into a `Iterable` element
    fn decode(&self, reader: &mut dyn Read, context: &Context) -> Result<Iterable<T>, io::Error> {
        unimplemented!()
    }

}

impl<T> CoderTestUtils for IterableCoder<T>
where
    T: Clone + fmt::Debug
{
    type InternalCoderType = T;

    fn parse_yaml_value(&self, value: &serde_yaml::Value) -> <IterableCoder<T> as CoderTestUtils>::InternalCoderType {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::any::Any;

    use bytes::{Buf, BufMut};
    use serde::Deserialize;
    use serde_yaml::{Deserializer, Value};

    // TODO: empty this list
    const UNSUPPORTED_CODERS: [&'static str; 15] = [
        "beam:coder:bool:v1",
        // TODO: fix StrUtf8Coder for unicode values and retest it
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
                },
                CoderTypeDiscriminants::StrUtf8 => {
                    let c = StrUtf8Coder::new();
                    run_unnested::<StrUtf8Coder, String>(&c, nested, &spec);
                }
                _ => unimplemented!(),
            }
        }
    }

    fn run_unnested<'a, C, E>(coder: &(dyn Any + 'a), _nested: bool, spec: &Value)
    where
        C: CoderI<E> + CoderTestUtils + CoderTestUtils<InternalCoderType = E> + 'a,
        E: Clone + std::fmt::Debug + PartialEq,
    {
        let examples = spec.get("examples").unwrap().as_mapping().unwrap();

        for (expected, original) in examples.iter() {
            // TODO: test coders for both Context types
            run_case::<C, E>(coder, expected, original);
        }
    }

    fn run_case<'a, C, E>(coder: &(dyn Any + 'a), expected_encoded: &Value, original: &Value)
    where
        C: CoderI<E> + CoderTestUtils + CoderTestUtils<InternalCoderType = E> + 'a,
        E: Clone + std::fmt::Debug + PartialEq,
    {
        let c: &C = coder.downcast_ref::<C>().unwrap();

        let expected_enc = expected_encoded.as_str().unwrap().as_bytes().to_vec();

        let mut writer = vec![].writer();
        let mut reader = expected_enc.reader();

        // TODO: revisit when context gets fully implemented
        let context = Context::WholeStream;
        
        let expected_dec = c.parse_yaml_value(original);

        c.encode(expected_dec.clone(), &mut writer, &context)
            .unwrap();
        let encoded = writer.into_inner();

        let decoded = c.decode(&mut reader, &context).unwrap();

        println!("\n---------\nCoder type: {:?}", c.get_coder_type());
        println!(
            "\nExpected encoded: {:?}\nGenerated encoded: {:?}\n\nExpected decoded: {:?}\nGenerated decoded: {:?}",
            expected_enc.as_slice(), encoded, expected_dec, decoded
        );

        assert_eq!(encoded, expected_enc.as_slice());
        assert_eq!(decoded, expected_dec);
    }
}
