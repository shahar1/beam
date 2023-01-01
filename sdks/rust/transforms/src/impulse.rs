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
use std::sync::Arc;

use coders::required_coders::BytesCoder;
use internals::pipeline::Pipeline;
use internals::pvalue::{PTransform, PType, PValue};
use internals::{pipeline::get_pcollection_name, urns};
use proto::beam::pipeline as proto_pipeline;

pub struct Impulse {
    urn: &'static str,
}

impl Impulse {
    pub fn new() -> Self {
        Self {
            urn: "beam:transform:impulse:v1",
        }
    }
}

// Input type should be never(!)
// https://github.com/rust-lang/rust/issues/35121
impl PTransform<bool, Vec<u8>> for Impulse {
    fn expand(&self, input: PValue<bool>) -> PValue<Vec<u8>> {
        assert!(*input.get_type() == PType::Root);

        let pcoll_name = get_pcollection_name();

        let coder_id = input.register_pipeline_coder_proto(proto_pipeline::Coder {
            spec: Some(proto_pipeline::FunctionSpec {
                urn: String::from(coders::urns::BYTES_CODER_URN),
                payload: Vec::with_capacity(0),
            }),
            component_coder_ids: Vec::with_capacity(0),
        });

        input.register_pipeline_coder::<BytesCoder, Vec<u8>>(Box::new(BytesCoder::new()));

        let output_proto = proto_pipeline::PCollection {
            unique_name: pcoll_name.clone(),
            coder_id: "placeholder".to_string(),
            is_bounded: proto_pipeline::is_bounded::Enum::Bounded as i32,
            windowing_strategy_id: "placeholder".to_string(),
            display_data: Vec::with_capacity(0),
        };

        let impulse_proto = proto_pipeline::PTransform {
            unique_name: "impulse".to_string(),
            spec: Some(proto_pipeline::FunctionSpec {
                urn: String::from(urns::DATA_INPUT_URN),
                payload: urns::IMPULSE_BUFFER.to_vec(),
            }),
            subtransforms: Vec::with_capacity(0),
            inputs: HashMap::with_capacity(0),
            outputs: HashMap::from([("out".to_string(), pcoll_name.clone())]),
            display_data: Vec::with_capacity(0),
            environment_id: "".to_string(),
            annotations: HashMap::with_capacity(0),
        };

        input.register_pipeline_proto_transform(impulse_proto);

        PValue::new(
            PType::PCollection,
            pcoll_name,
            output_proto,
            input.get_pipeline_arc(),
        )
    }

    fn expand_internal(
        &self,
        input: PValue<bool>,
        pipeline: Arc<Pipeline>,
        transform_proto: proto_pipeline::PTransform,
    ) -> PValue<Vec<u8>>
    where
        Self: Sized,
    {
        self.expand(input)
    }
}

impl Default for Impulse {
    fn default() -> Self {
        Self::new()
    }
}
