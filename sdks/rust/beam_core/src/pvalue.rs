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
use std::sync::{Arc, Mutex};

use coders::standard_coders::{BytesCoder, CoderI};
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
pub struct PValue {
    ptype: PType,
    name: String,
    pcoll_proto: proto_pipeline::PCollection,
    pipeline: Arc<Pipeline>,
}

impl PValue {
    pub fn new(
        ptype: PType,
        name: String,
        pcoll_proto: proto_pipeline::PCollection,
        pipeline: Arc<Pipeline>,
    ) -> Self {
        Self {
            ptype,
            name,
            pcoll_proto,
            pipeline,
        }
    }

    pub fn new_root(pipeline: Arc<Pipeline>) -> Self {
        let pcoll_name = "root".to_string();

        let proto_coder_id = pipeline.register_coder_proto(proto_pipeline::Coder {
            spec: Some(proto_pipeline::FunctionSpec {
                urn: String::from(coders::standard_coders::BYTES_CODER_URN),
                payload: Vec::with_capacity(0),
            }),
            component_coder_ids: Vec::with_capacity(0),
        });

        pipeline.register_coder::<BytesCoder, Vec<u8>>(Box::new(BytesCoder::new()));

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

        pipeline.register_proto_transform(impulse_proto);

        PValue::new(PType::Root, pcoll_name, output_proto, pipeline.clone())
    }

    pub fn register_pipeline_coder<'a, C: CoderI<E> + 'a, E>(
        &self,
        coder: Box<dyn Any + Send + 'a>,
    ) -> TypeId {
        self.pipeline.register_coder::<C, E>(coder)
    }

    pub fn register_pipeline_coder_proto(&self, coder_proto: proto_pipeline::Coder) -> String {
        self.pipeline.register_coder_proto(coder_proto)
    }

    pub fn register_pipeline_proto_transform(&self, transform: proto_pipeline::PTransform) {
        self.pipeline.register_proto_transform(transform)
    }

    pub fn get_pipeline_arc(&self) -> Arc<Pipeline> {
        self.pipeline.clone()
    }

    pub fn get_type(&self) -> &PType {
        &self.ptype
    }

    pub fn apply<F>(self, transform: F) -> PValue
    where
        F: PTransform + Sized,
    {
        transform.expand(self)
    }

    pub fn map(&self, callable: impl Fn() -> PValue) -> PValue {
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
    coders: Mutex<HashMap<TypeId, Box<dyn Any + Send>>>,

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

    pub fn get_proto(&self) -> Arc<Mutex<proto_pipeline::Pipeline>> {
        unimplemented!()
    }

    pub fn get_coder<C: CoderI<E> + Clone + 'static, E>(&self, coder_type: &TypeId) -> C {
        let pipeline_coders = self.coders.lock().unwrap();

        let coder = pipeline_coders.get(coder_type).unwrap();
        coder.downcast_ref::<C>().unwrap().clone()
    }

    pub fn register_coder<C: CoderI<E> + 'a, E>(&self, coder: Box<dyn Any + Send + 'a>) -> TypeId {
        let mut coders = self.coders.lock().unwrap();
        let concrete_coder = coder.downcast_ref::<C>().unwrap();
        let concrete_coder_type_id = concrete_coder.type_id();

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

    pub fn register_proto_transform(&self, transform: proto_pipeline::PTransform) {
        let mut pipeline_proto = self.proto.lock().unwrap();

        pipeline_proto
            .components
            .as_mut()
            .unwrap()
            .transforms
            .insert(transform.unique_name.clone(), transform);
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
