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

use std::marker::PhantomData;
use std::sync::Arc;

use crate::internals::pipeline::Pipeline;
use crate::internals::pvalue::{PTransform, PValue};
use crate::internals::serialize;
use crate::internals::urns;

use crate::proto::beam_api::pipeline as proto_pipeline;

pub struct DoFn;

impl DoFn {
    pub fn process() {
        todo!()
    }

    pub fn start_bundle() {
        todo!()
    }

    pub fn finish_bundle() {
        todo!()
    }
}

pub struct ParDo<T, O> {
    payload: String,
    phantom_t: PhantomData<T>,
    phantom_o: PhantomData<O>,
}

// TODO: Is the Sync + Send stuff needed?
impl<T: 'static, O: 'static> ParDo<T, O> {
    // TODO: These should correspond to methods on PCollection<T> (but not on PValue).
    pub fn from_map(func: fn(&T) -> O) -> Self {
        Self::from_dyn_map(Box::new(func))
    }
    pub fn from_dyn_map(func: Box<dyn Fn(&T) -> O + Sync + Send>) -> Self {
        Self::from_dyn_flat_map(Box::new(move |x: &T| -> Vec<O> { vec![func(x)] }))
    }
    pub fn from_flat_map<I: IntoIterator<Item = O> + 'static>(func: fn(&T) -> I) -> Self {
        Self::from_dyn_flat_map(Box::new(func))
    }
    pub fn from_dyn_flat_map<I: IntoIterator<Item = O> + 'static>(
        func: Box<dyn Fn(&T) -> I + Sync + Send>,
    ) -> Self {
        Self {
            payload: serialize::serialize_fn::<serialize::GenericDoFn>(Box::new(
                serialize::to_generic_dofn_dyn(func),
            )),
            phantom_t: PhantomData,
            phantom_o: PhantomData,
        }
    }
}

impl<T: Clone + std::marker::Send, O: Clone + std::marker::Send> PTransform<T, O> for ParDo<T, O> {
    fn expand_internal(
        &self,
        _input: &PValue<T>, // really a PCollection<T>
        pipeline: Arc<Pipeline>,
        transform_proto: &mut proto_pipeline::PTransform,
    ) -> PValue<O> // really a PCollection<O>
    {
        // Update the spec to say how it's created.
        transform_proto.spec = Some(proto_pipeline::FunctionSpec {
            urn: urns::PAR_DO_URN.to_string(),
            payload: self.payload.clone().into(),
        });
        pipeline.create_pcollection_internal("".to_string(), pipeline.clone())
    }
}
