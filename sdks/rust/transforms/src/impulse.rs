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

use beam_core::pvalue::{get_pcollection_name, PTransform, PType, PValue};
use coders::standard_coders::BytesCoder;
use internals::urns;
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

impl PTransform for Impulse {
    fn expand(mut self, input: PValue) -> PValue {
        assert!(*input.get_type() == PType::Root);

        // TODO: move this elsewhere?
        // TODO: import value from StandardPTransforms_Primitives in proto.pipelines
        self.urn = "beam:transform:impulse:v1";

        let pcoll_name = get_pcollection_name();

        let coder_id = input.register_pipeline_coder_proto(proto_pipeline::Coder {
            spec: Some(proto_pipeline::FunctionSpec {
                urn: String::from(coders::standard_coders::BYTES_CODER_URN),
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
}

impl Default for Impulse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use runners::runner::Runner;
    use std::any::Any;

    #[test]
    fn run_impulse_expansion() {
        let runner = Runner::new();
        let root = runner.run();
        let pcoll = root.apply(Impulse::new());

        // TODO: test proto coders
        // let pipeline_proto = runner.pipeline.proto.lock().unwrap();
        // let proto_coders = pipeline_proto.components.unwrap().coders;
        // let coder = *proto_coders
        //     .get(&root_clone.pcoll_proto.coder_id)
        //     .unwrap();

        let bytes_coder_type_id = BytesCoder::new().type_id();
        let coder = runner
            .get_pipeline_arc()
            .get_coder::<BytesCoder, Vec<u8>>(&bytes_coder_type_id);

        assert_eq!(*pcoll.get_type(), PType::PCollection);
        assert_eq!(coder.type_id(), bytes_coder_type_id);
    }
}
