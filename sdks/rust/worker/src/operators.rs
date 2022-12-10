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

use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use proto::beam::fn_execution::{ProcessBundleDescriptor, RemoteGrpcPort};
use proto::beam::pipeline::PTransform;

use crate::data::MultiplexingDataChannel;

type OperatorMap = HashMap<String, Box<dyn IOperator + Send + Sync>>;

static OPERATORS_BY_URN: Lazy<Mutex<OperatorMap>> = Lazy::new(|| {
    let m: OperatorMap = HashMap::new();
    Mutex::new(m)
});

#[derive(Clone, fmt::Debug)]
pub struct Operator {
    data: OperatorData,
    receivers: Vec<Receiver>,
}

#[derive(Clone, fmt::Debug)]
pub enum OperatorData {
    Create(CreateOperator),
    Recording(RecordingOperator),
}

#[derive(Clone, fmt::Debug)]
pub struct CreateOperator {
    // TODO: placeholder for Any
    data: String,
}

#[derive(Clone, fmt::Debug)]
pub struct RecordingOperator {
    log: Vec<String>,
    transform_id: String,
}

pub fn create_operator(transform_id: &str, context: OperatorContext) -> Operator {
    unimplemented!()

    // let transform = context
    //     .descriptor
    //     .transforms
    //     .get(transform_id)
    //     .expect("Transform ID not found");

    // for pcoll_id in transform.outputs.values() {
    //     (context.get_receiver)(pcoll_id.clone());
    // }

    // let operators_by_urn = OPERATORS_BY_URN.lock().unwrap();

    // let spec = transform
    //     .spec
    //     .as_ref()
    //     .unwrap_or_else(|| panic!("Transform {} has no spec", transform_id));

    // let op = operators_by_urn
    //     .get(&spec.urn)
    //     .unwrap_or_else(|| panic!("Unknown transform type: {}", spec.urn));

    // op.new_op(transform_id, transform, &context)
}

#[allow(clippy::new_ret_no_self)]
pub trait IOperator: Send {
    fn new_op(
        &self,
        transform_id: &str,
        transform: &PTransform,
        context: &OperatorContext,
    ) -> Box<dyn IOperator>;

    fn start_bundle(&self);

    fn process(&self, wvalue: &WindowedValue);

    fn finish_bundle(&self);
}

impl fmt::Debug for dyn IOperator {
    fn fmt<'a>(&'a self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_tuple("IOperator").finish()
    }
}

#[derive(Clone, Debug)]
pub struct Receiver {
    operators: Vec<Operator>,
}

impl Receiver {
    pub fn new(operators: Vec<Operator>) -> Self {
        Receiver { operators }
    }

    // pub fn receive(&self, wvalue: &WindowedValue) {
    //     for op in &self.operators {
    //         op.process(wvalue);
    //     }
    // }
}

#[derive(Debug)]
pub struct WindowedValue {
    value: Box<dyn Any>,
}

pub struct OperatorContext<'bundle_processor> {
    pub descriptor: &'bundle_processor ProcessBundleDescriptor,
    pub get_receiver: Box<dyn Fn(String) -> Receiver + 'bundle_processor>,
    // get_data_channel: fn(&str) -> MultiplexingDataChannel,
    // get_bundle_id: String,
}

// ******* Operator definitions *******

pub mod test_operators {}
