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

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;

use coders::standard_coders::{BytesCoder, CoderI};
use internals::urns;
use proto::beam::pipeline as proto_pipeline;

const _CODER_ID_PREFIX: &str = "coder_";

// TODO: use something better...
pub fn get_pcollection_name() -> String {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let bad_id: usize = rng.gen();
    format!("ref_PCollection_{}", bad_id)
}

#[derive(Clone)]
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
// TODO: use strum discriminants on PValues instead of this
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PType {
    Root,
    PCollection,
    PValueArr,
    PValueMap,
}

pub struct Pipeline {
    proto: Mutex<proto_pipeline::Pipeline>,
    // TODO: use AnyCoder instead of Any
    coders: Mutex<HashMap<TypeId, Box<dyn Any>>>,

    coder_proto_counter: Mutex<usize>,
}

impl<'a> Pipeline {
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

        Pipeline {
            // TODO: maybe lock individual components instead of the entire proto
            // and/or switch to async mutex
            proto: Mutex::new(proto),
            // TODO: try to refactor to RwLock
            coders: Mutex::new(HashMap::new()),
            coder_proto_counter: Mutex::new(0),
        }
    }

    // TODO: revisit this for the long term and make it more consistent in the
    // short term (e.g. a Box input will mess up the type for now)
    fn get_concrete_type_id(trait_obj: &dyn Any) -> TypeId {
        (*trait_obj).type_id()
    }

    pub fn register_coder<C: CoderI<E> + 'a, E>(&self, coder: Box<dyn Any + 'a>) -> TypeId {
        let mut coders = self.coders.lock().unwrap();
        let concrete_coder = coder.downcast_ref::<C>().unwrap();
        let concrete_coder_type_id = Pipeline::get_concrete_type_id(concrete_coder);

        for registered_type_id in coders.keys() {
            if *registered_type_id == concrete_coder_type_id {
                return *registered_type_id;
            }
        }

        coders.insert(concrete_coder_type_id, coder);

        concrete_coder_type_id
    }

    // TODO: review need for separate function vs register_coder
    pub fn register_coder_proto(&self, coder_proto: proto_pipeline::Coder) -> String {
        let mut pipeline_proto = self.proto.lock().unwrap();

        let proto_coders = &mut pipeline_proto.components.as_mut().unwrap().coders;

        for (coder_id, coder) in proto_coders.iter() {
            if *coder == coder_proto {
                return coder_id.clone();
            }
        }

        let mut coder_counter = self.coder_proto_counter.lock().unwrap();
        *coder_counter += 1;
        let new_coder_id = format!("{}{}", _CODER_ID_PREFIX, *coder_counter);
        proto_coders.insert(new_coder_id.clone(), coder_proto);

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

        let coder_id = input.pipeline.register_coder_proto(proto_pipeline::Coder {
            spec: Some(proto_pipeline::FunctionSpec {
                urn: String::from(coders::standard_coders::BYTES_CODER_URN),
                payload: Vec::with_capacity(0),
            }),
            component_coder_ids: Vec::with_capacity(0),
        });

        input
            .pipeline
            .register_coder::<BytesCoder, Vec<u8>>(Box::new(BytesCoder::new()));
        // .register_coder(Coder::Bytes(BytesCoder::new()));

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

        drop(pipeline_proto);

        PValue::new(PType::PCollection, pcoll_name, output_proto, input.pipeline)
    }
}

impl Default for Impulse {
    fn default() -> Self {
        Self::new()
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

        let proto_coder_id = self.pipeline.register_coder_proto(proto_pipeline::Coder {
            spec: Some(proto_pipeline::FunctionSpec {
                urn: String::from(coders::standard_coders::BYTES_CODER_URN),
                payload: Vec::with_capacity(0),
            }),
            component_coder_ids: Vec::with_capacity(0),
        });

        self.pipeline
            .register_coder::<BytesCoder, Vec<u8>>(Box::new(BytesCoder::new()));

        let output_proto = proto_pipeline::PCollection {
            unique_name: pcoll_name.clone(),
            coder_id: proto_coder_id,
            is_bounded: proto_pipeline::is_bounded::Enum::Bounded as i32,
            windowing_strategy_id: "placeholder".to_string(),
            display_data: Vec::with_capacity(0),
        };

        let impulse_proto = proto_pipeline::PTransform {
            unique_name: "root".to_string(),
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

        PValue::<'a>::new(PType::Root, pcoll_name, output_proto, &self.pipeline)
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let bytes_coder_type = Pipeline::get_concrete_type_id(&BytesCoder::new());

        let pipeline_coders = runner.pipeline.coders.lock().unwrap();

        let coder = pipeline_coders.get(&bytes_coder_type).unwrap().as_ref();

        assert_eq!(*pcoll.get_type(), PType::PCollection);
        assert_eq!(coder.type_id(), bytes_coder_type);
    }
}
