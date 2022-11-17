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

pub fn create_operator(transform_id: &str, context: OperatorContext) -> Box<dyn IOperator> {
    let transform = context
        .descriptor
        .transforms
        .get(transform_id)
        .expect("Transform ID not found");

    // Ensure receivers are eagerly created
    for pcoll_id in transform.outputs.values() {
        (context.get_receiver)(pcoll_id.clone());
    }

    let operators_by_urn = OPERATORS_BY_URN.lock().unwrap();

    let spec = transform
        .spec
        .as_ref()
        .unwrap_or_else(|| panic!("Transform {} has no spec", transform_id));

    let op = operators_by_urn
        .get(&spec.urn)
        .unwrap_or_else(|| panic!("Unknown transform type: {}", spec.urn));

    op.new_op(transform_id, transform, &context)
}

#[allow(clippy::new_ret_no_self)]
pub trait IOperator {
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

#[derive(Debug)]
pub struct Receiver {
    operators: Vec<Box<dyn IOperator>>,
}

impl Receiver {
    pub fn new(operators: Vec<Box<dyn IOperator>>) -> Self {
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
    value: Box<dyn Any>,
}

pub struct OperatorContext {
    descriptor: ProcessBundleDescriptor,
    get_receiver: fn(String) -> Receiver,
    get_data_channel: fn(&str) -> MultiplexingDataChannel,
    get_bundle_id: String,
}
