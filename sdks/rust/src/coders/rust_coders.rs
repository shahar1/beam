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

mod unit_coder;
pub use unit_coder::UnitCoder;

use std::fmt;
use std::marker::PhantomData;

use crate::coders::standard_coders::*;
use crate::coders::urns::*;
use crate::coders::Coder;
use crate::coders::CoderForPipeline;
use crate::coders::CoderUrn;
use crate::elem_types::ElemType;

#[derive(Eq, PartialEq)]
pub struct GeneralObjectCoder<T> {
    phantom: PhantomData<T>,
}

impl CoderUrn for GeneralObjectCoder<String> {
    const URN: &'static str = GENERAL_OBJECT_CODER_URN;
}

impl Coder for GeneralObjectCoder<String> {
    fn new(component_coders: Vec<Box<dyn Coder>>) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn encode(
        &self,
        element: &dyn ElemType,
        writer: &mut dyn std::io::Write,
        context: &crate::coders::Context,
    ) -> Result<usize, std::io::Error> {
        let marker = "S".as_bytes();
        writer.write_all(marker).unwrap();

        StrUtf8Coder::default().encode(element, writer, context)
    }

    fn decode(
        &self,
        reader: &mut dyn std::io::Read,
        context: &crate::coders::Context,
    ) -> Result<Box<dyn ElemType>, std::io::Error> {
        let marker: &mut [u8; 1] = &mut [0; 1];
        reader.read_exact(marker)?;

        if marker == "Z".as_bytes() {
            todo!()
        }

        StrUtf8Coder::default().decode(reader, context)
    }
}

impl CoderForPipeline for GeneralObjectCoder<String> {
    fn component_coder_urns() -> Vec<super::CoderUrnTree> {
        vec![]
    }
}

impl<T> Default for GeneralObjectCoder<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }
}

impl<T> fmt::Debug for GeneralObjectCoder<T> {
    fn fmt(&self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_struct("GeneralObjectCoder")
            .field("urn", &GENERAL_OBJECT_CODER_URN)
            .finish()
    }
}
