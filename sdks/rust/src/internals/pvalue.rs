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
use std::marker::PhantomData;
use std::sync::Arc;

use crate::coders::coders::CoderI;
use crate::coders::required_coders::BytesCoder;
use crate::proto::beam_api::pipeline as proto_pipeline;

use crate::internals::pipeline::Pipeline;

// T should be never(!) for Root
// https://github.com/rust-lang/rust/issues/35121
#[derive(Clone)]
pub struct PValue<T>
where
    T: Clone + Send,
{
    id: String,
    ptype: PType,
    pipeline: Arc<Pipeline>,

    phantom: PhantomData<T>,
}

impl<T> PValue<T>
where
    T: Clone + Send,
{
    pub fn new(ptype: PType, pipeline: Arc<Pipeline>, id: String) -> Self {
        Self {
            id,
            ptype,
            pipeline,

            phantom: PhantomData::default(),
        }
    }

    pub fn new_root(pipeline: Arc<Pipeline>) -> Self {
        PValue::new(PType::Root, pipeline, crate::internals::utils::get_bad_id())
    }

    pub fn new_array(pcolls: &[PValue<T>]) -> Self {
        PValue::new(
            PType::PValueArr,
            pcolls[0].clone().pipeline,
            pcolls
                .iter()
                .map(|pcoll| -> String { pcoll.id.clone() })
                .collect::<Vec<String>>()
                .join(","),
        )
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
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn apply<F, Out>(self, transform: F) -> PValue<Out>
    where
        Out: Clone + Send,
        F: PTransform<T, Out> + Send,
    {
        self.pipeline
            .apply_transform(transform, &self, self.pipeline.clone())
    }

    // pub fn map(&self, callable: impl Fn() -> PValue) -> PValue {
    //     todo!()
    // }
}

/// Returns a PValue as a flat object with string keys and PCollection id values.
///
/// The full set of PCollections reachable by this PValue will be returned,
/// with keys corresponding roughly to the path taken to get there
pub fn flatten_pvalue<T>(pvalue: PValue<T>, prefix: Option<String>) -> HashMap<String, String>
where
    T: Clone + Send,
{
    let mut result: HashMap<String, String> = HashMap::new();
    match pvalue.ptype {
        PType::PCollection => match prefix {
            Some(pr) => {
                result.insert(pr, pvalue.get_id());
            }
            None => {
                result.insert("main".to_string(), pvalue.get_id());
            }
        },
        PType::PValueArr => {
            // TODO: Remove this hack, PValues can have multiple ids.
            for (i, id) in pvalue.get_id().split(",").enumerate() {
                result.insert(i.to_string(), id.to_string());
            }
        }
        PType::PValueMap => todo!(),
        PType::Root => {}
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
    In: Clone + Send,
    Out: Clone + Send,
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
        transform_proto: &mut proto_pipeline::PTransform,
    ) -> PValue<Out>
    where
        Self: Sized,
    {
        self.expand(input)
    }
}
