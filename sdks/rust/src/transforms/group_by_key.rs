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

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::coders::CoderUrnTree;
use crate::elem_types::kv::KV;
use crate::elem_types::ElemType;
use crate::internals::pipeline::Pipeline;
use crate::internals::pvalue::{PTransform, PValue};
use crate::internals::serialize;
use crate::internals::urns;

use crate::proto::pipeline::v1 as pipeline_v1;

pub struct GroupByKey<K, V> {
    payload: String,
    phantom: PhantomData<(K, V)>,
}

pub struct KeyExtractor<V: ElemType>(PhantomData<V>);

// TODO: Use coders to allow arbitrary keys.
impl<V: ElemType + Clone + fmt::Debug> Default for GroupByKey<String, V> {
    fn default() -> Self {
        Self {
            payload: serialize::store_key_extractor(KeyExtractor::<V>(PhantomData)),
            phantom: PhantomData,
        }
    }
}

// TODO: The return value should be something like dyn IntoIterator<Item = V, IntoIter = Box<dyn Iterator<Item = V>>> + Clone + Sync + Send + 'static,
// to avoid requiring it to be in memory.
impl<K, V> PTransform<KV<K, V>, KV<K, Vec<V>>> for GroupByKey<K, V>
where
    K: ElemType + Clone + fmt::Debug,
    V: ElemType + Clone + fmt::Debug,
{
    fn expand(
        &self,
        _input: &PValue<KV<K, V>>, // really a PCollection
        pipeline: Arc<Pipeline>,
        coder_urn: &CoderUrnTree,
        transform_proto: &mut pipeline_v1::PTransform,
    ) -> PValue<KV<K, Vec<V>>> // really a PCollection
    {
        transform_proto.spec = Some(pipeline_v1::FunctionSpec {
            urn: urns::GROUP_BY_KEY_URN.to_string(),
            payload: self.payload.clone().into(),
        });
        pipeline.create_pcollection_internal(coder_urn, pipeline.clone())
    }
}
