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

use std::iter::Iterator;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::internals::pipeline::Pipeline;
use crate::internals::pvalue::{PTransform, PValue};
use crate::internals::serialize;
use crate::internals::urns;

use crate::proto::beam_api::pipeline as proto_pipeline;

pub struct GroupByKey<K, V> {
    payload: String,
    phantom_k: PhantomData<K>,
    phantom_v: PhantomData<V>,
}

// TODO: Use coders to allow arbitrary keys.
impl<V: Clone + Sync + Send + 'static> GroupByKey<String, V> {
    pub fn new() -> Self {
        Self {
            payload: serialize::serialize_fn::<Box<dyn serialize::KeyExtractor>>(Box::new(
                Box::new(serialize::TypedKeyExtractor::<V>::default()),
            )),
            phantom_k: PhantomData,
            phantom_v: PhantomData,
        }
    }
}

// TODO: The return value should be something like dyn IntoIterator<Item = V, IntoIter = Box<dyn Iterator<Item = V>>> + Clone + Sync + Send + 'static,
// to avoid requiring it to be in memory.
impl<K: Clone + std::marker::Send, V: Clone + std::marker::Send> PTransform<(K, V), (K, Vec<V>)>
    for GroupByKey<K, V>
{
    fn expand_internal(
        &self,
        _input: &PValue<(K, V)>, // really a PCollection
        pipeline: Arc<Pipeline>,
        transform_proto: &mut proto_pipeline::PTransform,
    ) -> PValue<(K, Vec<V>)> // really a PCollection
    {
        transform_proto.spec = Some(proto_pipeline::FunctionSpec {
            urn: urns::GROUP_BY_KEY_URN.to_string(),
            payload: self.payload.clone().into(),
        });
        pipeline.create_pcollection_internal("".to_string(), pipeline.clone())
    }
}
