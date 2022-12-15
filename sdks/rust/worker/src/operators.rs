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
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use internals::urns;
use proto::beam::fn_execution::{ProcessBundleDescriptor, RemoteGrpcPort};
use proto::beam::pipeline::PTransform;

use crate::data::MultiplexingDataChannel;
use crate::sdk_worker::BundleProcessor;

type OperatorMap = HashMap<&'static str, OperatorDiscriminants>;

static OPERATORS_BY_URN: Lazy<Mutex<OperatorMap>> = Lazy::new(|| {
    // TODO: these will have to be parameterized depending on things such as the runner used
    let m: OperatorMap = HashMap::from([
        // Test operators
        (urns::CREATE_URN, OperatorDiscriminants::Create),
        (urns::RECORDING_URN, OperatorDiscriminants::Recording),
        (urns::PARTITION_URN, OperatorDiscriminants::Partitioning),
        // Production operators
        (urns::DATA_INPUT_URN, OperatorDiscriminants::DataSource),
    ]);

    Mutex::new(m)
});

pub trait OperatorI {
    fn new(
        transform_id: Arc<String>,
        transform: Arc<PTransform>,
        context: Arc<OperatorContext>,
        operator_discriminant: Arc<OperatorDiscriminants>,
    ) -> Self
    where
        Self: Sized;

    fn start_bundle(&self);

    fn process(&self, wvalue: &WindowedValue);

    fn finish_bundle(&self) {
        unimplemented!()
    }
}

#[derive(Clone, fmt::Debug, EnumDiscriminants)]
pub enum Operator {
    // Test operators
    Create(CreateOperator),
    Recording(RecordingOperator),
    Partitioning,

    // Production operators
    DataSource,

    Placeholder,
}

impl OperatorI for Operator {
    fn new(
        transform_id: Arc<String>,
        transform: Arc<PTransform>,
        context: Arc<OperatorContext>,
        operator_discriminant: Arc<OperatorDiscriminants>,
    ) -> Self {
        match operator_discriminant.as_ref() {
            OperatorDiscriminants::Create => Operator::Create(CreateOperator::new(
                transform_id,
                transform,
                context,
                operator_discriminant,
            )),
            _ => unimplemented!(),
        }
    }

    fn start_bundle(&self) {
        match self {
            Operator::Create(create_op) => create_op.start_bundle(),
            Operator::Recording(recording_op) => recording_op.start_bundle(),
            _ => unimplemented!(),
        };
    }

    fn process(&self, wvalue: &WindowedValue) {
        match self {
            Operator::Create(create_op) => {
                unimplemented!()
                // create_op.process()
            }
            Operator::Recording(recording_op) => {
                unimplemented!()
                // recording_op.process()
            }
            _ => unimplemented!(),
        };
    }
}

#[derive(Clone, Debug)]
pub struct CreateOperator {
    transform_id: Arc<String>,
    transform: Arc<PTransform>,
    context: Arc<OperatorContext>,
    operator_discriminant: Arc<OperatorDiscriminants>,

    data: Vec<u8>,
}

impl OperatorI for CreateOperator {
    fn new(
        transform_id: Arc<String>,
        transform: Arc<PTransform>,
        context: Arc<OperatorContext>,
        operator_discriminant: Arc<OperatorDiscriminants>,
    ) -> Self {
        let data = transform
            .as_ref()
            .spec
            .as_ref()
            .expect("No spec found for transform")
            .payload
            .clone();

        Self {
            transform_id,
            transform,
            context,
            operator_discriminant,
            data,
        }
    }

    fn start_bundle(&self) {
        unimplemented!()
    }

    fn process(&self, wvalue: &WindowedValue) {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct RecordingOperator {
    transform_id: Arc<String>,
    transform: Arc<PTransform>,
    context: Arc<OperatorContext>,
    operator_discriminant: Arc<OperatorDiscriminants>,

    log: Vec<String>,
}

impl OperatorI for RecordingOperator {
    fn new(
        transform_id: Arc<String>,
        transform: Arc<PTransform>,
        context: Arc<OperatorContext>,
        operator_discriminant: Arc<OperatorDiscriminants>,
    ) -> Self {
        Self {
            transform_id,
            transform,
            context,
            operator_discriminant: operator_discriminant,
            log: Vec::new(),
        }
    }

    fn start_bundle(&self) {
        unimplemented!()
    }

    fn process(&self, wvalue: &WindowedValue) {
        unimplemented!()
    }
}

pub fn create_operator(transform_id: &str, context: OperatorContext) -> Operator {
    let descriptor: &ProcessBundleDescriptor = context.descriptor.as_ref();

    let transform = descriptor
        .transforms
        .get(transform_id)
        .expect("Transform ID not found");

    for pcoll_id in transform.outputs.values() {
        (context.get_receiver)(context.bundle_processor.clone(), pcoll_id.clone());
    }

    let operators_by_urn = OPERATORS_BY_URN.lock().unwrap();

    let spec = transform
        .spec
        .as_ref()
        .unwrap_or_else(|| panic!("Transform {} has no spec", transform_id));

    let op_discriminant = operators_by_urn
        .get(spec.urn.as_str())
        .unwrap_or_else(|| panic!("Unknown transform type: {}", spec.urn));

    match op_discriminant {
        OperatorDiscriminants::Create => Operator::Placeholder,
        OperatorDiscriminants::Recording => Operator::Placeholder,
        _ => unimplemented!(),
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

    pub fn receive(&self, wvalue: &WindowedValue) {
        for op in &self.operators {
            op.process(wvalue);
        }
    }
}

#[derive(Debug)]
pub struct WindowedValue {
    // TODO: placeholder for Any
    value: Vec<u8>,
}

pub struct OperatorContext {
    pub descriptor: Arc<ProcessBundleDescriptor>,
    pub get_receiver: Box<dyn Fn(Arc<BundleProcessor>, String) -> Receiver + Send + Sync>,
    // get_data_channel: fn(&str) -> MultiplexingDataChannel,
    // get_bundle_id: String,
    pub bundle_processor: Arc<BundleProcessor>,
}

impl fmt::Debug for OperatorContext {
    fn fmt<'a>(&'a self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_struct("OperatorContext")
            .field("descriptor", &self.descriptor)
            .field("bundle_processor", &self.bundle_processor)
            .finish()
    }
}

// ******* Operator definitions *******

pub mod test_operators {}
