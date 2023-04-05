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

use std::{any::Any, fmt};

use super::group_by_key::GroupByKey;
use super::impulse::Impulse;
use super::pardo::ParDo;
use crate::{
    elem_types::{kv::KV, ElemType},
    internals::pvalue::{PTransform, PValue},
};

pub struct AssertEqualUnordered<T> {
    // compare with actual (sorted)
    expected_sorted: Vec<T>,
}

impl<T: Any + Clone + Ord> AssertEqualUnordered<T> {
    pub fn new(expected_slice: &[T]) -> Self {
        let mut expected_sorted = expected_slice.to_vec();
        expected_sorted.sort();
        Self { expected_sorted }
    }
}

impl<T: ElemType + PartialEq + Ord + fmt::Debug> PTransform<T, ()> for AssertEqualUnordered<T> {
    fn expand(&self, input: &PValue<T>) -> PValue<()> {
        // If input is empty, we still need an element to ensure the
        // assertion happens.
        let singleton = PValue::new_root(input.get_pipeline_arc())
            .apply(Impulse::new())
            .apply(ParDo::from_map(|_x| -> Option<T> { None }));
        let actual = input
            .clone()
            .apply(ParDo::from_map(|x: &T| -> Option<T> { Some(x.clone()) }));
        let expected = self.expected_sorted.clone();
        PValue::new_array(&[singleton, actual])
            .apply(ParDo::from_map(|x: &Option<T>| -> KV<String, Option<T>> {
                KV::new("".to_string(), x.clone())
            }))
            .apply(GroupByKey::default())
            .apply(ParDo::from_dyn_map(Box::new(
                move |kvs: &KV<String, Vec<Option<T>>>| {
                    let mut actual: Vec<T> = kvs
                        .as_values()
                        .iter()
                        .filter(|x| -> bool { x.is_some() })
                        .map(|x| -> T { x.clone().unwrap() })
                        .collect();
                    actual.sort();
                    assert!(
                        actual == expected,
                        "Actual values ({:?}) do not equal expected values ({:?}).",
                        actual,
                        expected
                    );
                },
            )))
    }
}
