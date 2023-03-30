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

use std::sync::Arc;

use crate::coders::required_coders::BytesCoder;
use crate::internals::pipeline::Pipeline;
use crate::internals::pvalue::{PTransform, PValue};
use crate::internals::urns::IMPULSE_URN;
use crate::proto::beam_api::pipeline as proto_pipeline;

pub struct Impulse {
    urn: &'static str,
}

impl Impulse {
    pub fn new() -> Self {
        Self { urn: IMPULSE_URN }
    }
}

// Input type should be never(!)
// https://github.com/rust-lang/rust/issues/35121
pub type Never = bool;

impl PTransform<Never, Vec<u8>> for Impulse {
    fn expand_internal(
        &self,
        _input: &PValue<Never>,
        pipeline: Arc<Pipeline>,
        transform_proto: &mut proto_pipeline::PTransform,
    ) -> PValue<Vec<u8>> {
        let spec = proto_pipeline::FunctionSpec {
            urn: self.urn.to_string(),
            payload: crate::internals::urns::IMPULSE_BUFFER.to_vec(), // Should be able to omit.
        };
        transform_proto.spec = Some(spec);

        pipeline.register_coder::<BytesCoder, Vec<u8>>(Box::new(BytesCoder::new()));
        pipeline.create_pcollection_internal(
            pipeline.register_coder_proto(proto_pipeline::Coder {
                spec: Some(proto_pipeline::FunctionSpec {
                    urn: String::from(crate::coders::urns::BYTES_CODER_URN),
                    payload: Vec::with_capacity(0),
                }),
                component_coder_ids: Vec::with_capacity(0),
            }),
            pipeline.clone(),
        )
    }
}

impl Default for Impulse {
    fn default() -> Self {
        Self::new()
    }
}
