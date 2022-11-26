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
use std::sync::Mutex;

use internals::urns;
use proto::beam::pipeline as proto_pipeline;

const _CODER_ID_PREFIX: &'static str = "coder_";

// TODO: use something better...
pub fn get_pcollection_name() -> String {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let bad_id: usize = rng.gen();
    format!("ref_PCollection_{}", bad_id)
}

pub struct PValue<'a> {
    ptype: PType,
    name: String,
    pcoll_proto: proto_pipeline::PCollection,
    pipeline: &'a Pipeline,
}

impl<'a> PValue<'a> {
    pub fn new(
        ptype: PType,
        name: String,
        pcoll_proto: proto_pipeline::PCollection,
        pipeline: &'a Pipeline,
    ) -> Self {
        Self {
            ptype,
            name,
            pcoll_proto,
            pipeline,
        }
    }

    pub fn get_type(&self) -> &PType {
        &self.ptype
    }

    fn apply<F>(self, transform: F) -> PValue<'a>
    where
        F: PTransform + Sized,
    {
        transform.expand(self)
    }

    fn map(&self, callable: impl Fn() -> PValue<'a>) -> PValue {
        unimplemented!()
    }
}

// Anonymous sum types would probably be better, if/when they become
// available. https://github.com/rust-lang/rfcs/issues/294
#[derive(Eq, PartialEq)]
pub enum PType {
    Root,
    PCollection,
    PValueArr,
    PValueMap,
}

pub struct Pipeline {
    proto: Mutex<proto_pipeline::Pipeline>,

    coder_counter: usize,
}

impl Pipeline {
    pub fn new() -> Self {
        let proto = proto_pipeline::Pipeline {
            components: Some(proto_pipeline::Components {
                transforms: HashMap::with_capacity(0),
                pcollections: HashMap::with_capacity(0),
                windowing_strategies: HashMap::with_capacity(0),
                coders: HashMap::with_capacity(0),
                environments: HashMap::with_capacity(0),
            }),
            root_transform_ids: Vec::with_capacity(0),
            display_data: Vec::with_capacity(0),
            requirements: Vec::with_capacity(0),
        };

        // TODO: maybe lock individual components instead of the entire proto
        // and/or switch to async mutex
        Pipeline {
            proto: Mutex::new(proto),
            coder_counter: 0,
        }
    }

    pub fn register_coder(&mut self, coder_proto: proto_pipeline::Coder) -> String {
        let mut pipeline_proto = self.proto.lock().unwrap();

        let coders = &mut pipeline_proto.components.as_mut().unwrap().coders;

        for (key, val) in coders.iter() {
            if *val == coder_proto {
                return key.clone();
            }
        }

        self.coder_counter += 1;
        let new_coder_id = format!("{}{}", _CODER_ID_PREFIX, self.coder_counter);
        coders.insert(new_coder_id.clone(), coder_proto);

        new_coder_id
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PTransform {
    fn expand(self, input: PValue) -> PValue
    where
        Self: Sized,
    {
        match input.get_type() {
            PType::Root => {
                panic!()
            }
            PType::PCollection => {
                unimplemented!()
            }
            _ => unimplemented!(),
        }
    }
}

pub struct Impulse {
    urn: &'static str,
}

impl PTransform for Impulse {
    fn expand(mut self, input: PValue) -> PValue {
        assert!(*input.get_type() == PType::Root);

        // TODO: move this elsewhere?
        // TODO: import value from StandardPTransforms_Primitives in proto.pipelines
        self.urn = "beam:transform:impulse:v1";

        let pcoll_name = get_pcollection_name();

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

        let mut pipeline_proto = input.pipeline.proto.lock().unwrap();

        pipeline_proto
            .components
            .as_mut()
            .unwrap()
            .transforms
            .insert(impulse_proto.unique_name.clone(), impulse_proto);

        PValue::new(PType::PCollection, pcoll_name, output_proto, input.pipeline)
    }
}

pub struct DoFn;

impl DoFn {
    pub fn process() {
        unimplemented!()
    }

    pub fn start_bundle() {
        unimplemented!()
    }

    pub fn finish_bundle() {
        unimplemented!()
    }
}

pub struct Runner {
    pipeline: Pipeline,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            pipeline: Pipeline::new(),
        }
    }

    pub fn run<'a>(&'a self) -> PValue {
        let pcoll_name = get_pcollection_name();

        let output_proto = proto_pipeline::PCollection {
            unique_name: pcoll_name.clone(),
            coder_id: "placeholder".to_string(),
            is_bounded: proto_pipeline::is_bounded::Enum::Bounded as i32,
            windowing_strategy_id: "placeholder".to_string(),
            display_data: Vec::with_capacity(0),
        };

        let impulse_proto = proto_pipeline::PTransform {
            unique_name: "impulse".to_string(),
            spec: None,
            subtransforms: Vec::with_capacity(0),
            inputs: HashMap::with_capacity(0),
            outputs: HashMap::from([("out".to_string(), pcoll_name.clone())]),
            display_data: Vec::with_capacity(0),
            environment_id: "".to_string(),
            annotations: HashMap::with_capacity(0),
        };

        let mut pipeline_proto = self.pipeline.proto.lock().unwrap();

        pipeline_proto
            .components
            .as_mut()
            .unwrap()
            .transforms
            .insert(impulse_proto.unique_name.clone(), impulse_proto);

        PValue::<'a>::new(PType::PCollection, pcoll_name, output_proto, &self.pipeline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_basic_ptransform() {
        let runner = Runner::new();
        runner.run();

        assert_eq!(1, 1);
    }
}
