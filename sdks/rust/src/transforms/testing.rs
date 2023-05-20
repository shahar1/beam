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

use super::group_by_key::GroupByKey;
use super::impulse::Impulse;
use super::pardo::ParDo;
use crate::{
    coders::urns::UNIT_CODER_URN,
    elem_types::{kv::KV, DefaultCoder, ElemType},
    internals::pvalue::{PTransform, PType, PValue},
};

pub struct AssertEqualUnordered<T> {
    // compare with actual (sorted)
    expected_sorted: Vec<T>,
}

impl<E: ElemType + Clone + Ord> AssertEqualUnordered<E> {
    pub fn new(expected_slice: &[E]) -> Self {
        let mut expected_sorted = expected_slice.to_vec();
        expected_sorted.sort();
        Self { expected_sorted }
    }
}

impl<E: ElemType + DefaultCoder + Clone + PartialEq + Ord + fmt::Debug> PTransform<E, ()>
    for AssertEqualUnordered<E>
{
    fn expand_internal(
        &self,
        input: &PValue<E>,
        _pipeline: std::sync::Arc<crate::internals::pipeline::Pipeline>,
        _out_coder_urn: &crate::coders::CoderUrnTree,
        _transform_proto: &mut crate::proto::pipeline_v1::PTransform,
    ) -> PValue<()>
    where
        Self: Sized,
    {
        // If input is empty, we still need an element to ensure the assertion happens.
        let dummy = dummy_root(input)
            .apply(Impulse::new())
            .apply(ParDo::from_map(|_x| -> Option<E> { None })); // None values are filtered out later.

        let actual = input.apply(ParDo::from_map(|x: &E| -> Option<E> { Some(x.clone()) }));

        let expected = self.expected_sorted.clone();

        PValue::new_array(&[dummy, actual])
            .apply(ParDo::from_map(|x: &Option<E>| -> KV<String, Option<E>> {
                KV::new("".to_string(), x.clone())
            }))
            .apply(GroupByKey::default())
            .apply(ParDo::from_map(move |kvs: &KV<String, Vec<Option<E>>>| {
                let mut actual: Vec<E> = kvs
                    .as_values()
                    .iter()
                    .filter(|x| -> bool { x.is_some() }) // filter-out the `dummy`
                    .map(|x| -> E { x.clone().unwrap() })
                    .collect();
                actual.sort();
                assert!(
                    actual == expected,
                    "Actual values ({:?}) do not equal expected values ({:?}).",
                    actual,
                    expected
                );
            }))
    }
}

fn dummy_root<T: ElemType + PartialEq + Ord + fmt::Debug>(input: &PValue<T>) -> PValue<()> {
    let pipeline = input.get_pipeline_arc();
    PValue::new(
        PType::Root,
        pipeline,
        crate::internals::utils::get_bad_id(),
        UNIT_CODER_URN.to_string(),
    )
}
