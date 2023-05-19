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

use std::iter;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::coders::CoderUrnTree;
use crate::elem_types::ElemType;
use crate::internals::pipeline::Pipeline;
use crate::internals::pvalue::{PTransform, PValue};
use crate::internals::serialize;
use crate::internals::urns;

use crate::proto::pipeline::v1 as pipeline_v1;

pub trait DoFn: Send + Sync {
    type In: ElemType;
    type Out: ElemType;
    type I: IntoIterator<Item = Self::Out>;

    fn process(&self, elem: &Self::In) -> Self::I;
    fn start_bundle(&self) {}
    fn finish_bundle(&self) {}
}

struct DoFnFromFlatmap<In, Out, I, F: Fn(&In) -> I>(F, PhantomData<(In, Out, I)>);

impl<
        In: ElemType,
        Out: ElemType,
        I: IntoIterator<Item = Out> + Send + Sync,
        F: Fn(&In) -> I + Send + Sync,
    > DoFn for DoFnFromFlatmap<In, Out, I, F>
{
    type In = In;
    type Out = Out;
    type I = I;

    fn process(&self, elem: &In) -> I {
        self.0(elem)
    }
}

pub struct ParDo<In, Out> {
    payload: String,
    phantom: PhantomData<(In, Out)>,
}

// TODO: Is the Sync + Send stuff needed?
impl<In: ElemType, Out: ElemType> ParDo<In, Out> {
    pub fn of<D: DoFn<In = In, Out = Out> + 'static>(do_fn: D) -> Self {
        Self {
            payload: serialize::store_do_fn(do_fn),
            phantom: PhantomData,
        }
    }

    // TODO: These should correspond to methods on PCollection<T> (but not on PValue).
    pub fn from_flat_map<
        I: IntoIterator<Item = Out> + Send + Sync + 'static,
        F: Fn(&In) -> I + Send + Sync + 'static,
    >(
        func: F,
    ) -> Self {
        Self::of(DoFnFromFlatmap(func, PhantomData))
    }

    pub fn from_map<F: Fn(&In) -> Out + Send + Sync + 'static>(func: F) -> Self {
        Self::from_flat_map(move |t| iter::once(func(t)))
    }
}

impl<In, Out> PTransform<In, Out> for ParDo<In, Out>
where
    In: ElemType,
    Out: ElemType + Clone,
{
    fn expand(
        &self,
        _input: &PValue<In>, // really a PCollection<T>
        pipeline: Arc<Pipeline>,
        out_coder_urn: &CoderUrnTree,
        transform_proto: &mut pipeline_v1::PTransform,
    ) -> PValue<Out> // really a PCollection<O>
    {
        // Update the spec to say how it's created.
        transform_proto.spec = Some(pipeline_v1::FunctionSpec {
            urn: urns::PAR_DO_URN.to_string(),
            payload: self.payload.as_bytes().to_owned(),
        });
        pipeline.create_pcollection_internal(out_coder_urn, pipeline.clone())
    }
}
