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
use std::marker::PhantomData;
use std::sync::Arc;

use crate::elem_types::ElemType;
use crate::proto::beam_api::pipeline as proto_pipeline;

use crate::internals::pipeline::Pipeline;

// TODO: remove field pcoll_proto.
// T should be never(!) for Root
// https://github.com/rust-lang/rust/issues/35121
#[derive(Clone)]
pub struct PValue<E>
where
    E: ElemType,
{
    id: String,
    ptype: PType,
    pcoll_proto: proto_pipeline::PCollection,
    pipeline: Arc<Pipeline>,

    phantom: PhantomData<E>,
}

impl<E> PValue<E>
where
    E: ElemType,
{
    pub fn new(
        ptype: PType,
        pcoll_proto: proto_pipeline::PCollection,
        pipeline: Arc<Pipeline>,
        id: String,
    ) -> Self {
        Self {
            id,
            ptype,
            pcoll_proto,
            pipeline,

            phantom: PhantomData::default(),
        }
    }

    pub fn new_root(pipeline: Arc<Pipeline>) -> Self {
        let pcoll_name = "root".to_string();

        let proto_coder_id = pipeline.register_coder_proto(proto_pipeline::Coder {
            spec: Some(proto_pipeline::FunctionSpec {
                urn: String::from(crate::coders::urns::BYTES_CODER_URN),
                payload: Vec::with_capacity(0),
            }),
            component_coder_ids: Vec::with_capacity(0),
        });

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
            outputs: HashMap::from([("out".to_string(), pcoll_name)]),
            display_data: Vec::with_capacity(0),
            environment_id: "".to_string(),
            annotations: HashMap::with_capacity(0),
        };

        pipeline.register_proto_transform(impulse_proto);

        PValue::new(
            PType::Root,
            output_proto,
            pipeline,
            crate::internals::utils::get_bad_id(),
        )
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
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn apply<F, Out>(self, transform: F) -> PValue<Out>
    where
        Out: ElemType,
        F: PTransform<E, Out> + Send,
    {
        self.pipeline
            .apply_transform(transform, &self, self.pipeline.clone())
    }

    // pub fn map(&self, callable: impl Fn() -> PValue) -> PValue {
    //     todo!()
    // }
}

/// Returns a PValue as a flat object with string keys and PCollection values.
///
/// The full set of PCollections reachable by this PValue will be returned,
/// with keys corresponding roughly to the path taken to get there
pub fn flatten_pvalue<T>(pvalue: PValue<T>, prefix: Option<String>) -> HashMap<String, PValue<T>>
where
    T: ElemType,
{
    let mut result: HashMap<String, PValue<T>> = HashMap::new();
    match pvalue.ptype {
        PType::PCollection => match prefix {
            Some(pr) => {
                result.insert(pr, pvalue);
            }
            None => {
                result.insert("main".to_string(), pvalue);
            }
        },
        PType::PValueArr => todo!(),
        PType::PValueMap => todo!(),
        PType::Root => {
            result.insert("main".to_string(), pvalue);
        }
    }

    result
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

// TODO: move this to transforms directory
pub trait PTransform<In, Out>
where
    In: ElemType,
    Out: ElemType,
{
    fn expand(&self, input: &PValue<In>) -> PValue<Out>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn expand_internal(
        &self,
        input: &PValue<In>,
        pipeline: Arc<Pipeline>,
        transform_proto: proto_pipeline::PTransform,
    ) -> PValue<Out>
    where
        Self: Sized,
    {
        self.expand(input)
    }
}
