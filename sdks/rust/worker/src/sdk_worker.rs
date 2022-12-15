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

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, RwLock};

use http::Uri;
use proto::beam::pipeline::PTransform;
use serde_json;
use tonic::transport::Channel;

use proto::beam::fn_execution::{
    beam_fn_control_client::BeamFnControlClient, InstructionRequest, InstructionResponse,
    ProcessBundleDescriptor, ProcessBundleResponse, ProcessBundleSplitRequest,
    ProcessBundleSplitResponse,
};

use crate::operators::{create_operator, Operator, OperatorContext, OperatorI, Receiver};

#[derive(Debug)]
pub struct Worker {
    control_client: Arc<BeamFnControlClient<Channel>>,

    process_bundle_descriptors: HashMap<String, ProcessBundleDescriptor>,
    bundle_processors: HashMap<String, BundleProcessor>,
    active_bundle_processors: HashMap<String, BundleProcessor>,
    id: String,
    endpoints: WorkerEndpoints,
    options: HashMap<String, String>,
}

impl Worker {
    pub async fn new(id: String, endpoints: WorkerEndpoints) -> Arc<Mutex<Worker>> {
        // TODO: parse URIs in the endpoint struct
        let channel = Channel::builder(endpoints.get_endpoint().parse::<Uri>().unwrap())
            .connect()
            .await
            .expect("Failed to connect to worker");

        Arc::new(Mutex::new(Self {
            control_client: Arc::new(BeamFnControlClient::new(channel)),

            process_bundle_descriptors: HashMap::new(),
            bundle_processors: HashMap::new(),
            active_bundle_processors: HashMap::new(),
            id,
            endpoints,
            options: HashMap::new(),
        }))
    }

    // TODO
    pub fn stop(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct WorkerEndpoints {
    control_endpoint_url: Option<String>,
}

impl WorkerEndpoints {
    pub fn new(control_endpoint_url: Option<String>) -> Self {
        Self {
            control_endpoint_url,
        }
    }

    pub fn get_endpoint(&self) -> &str {
        self.control_endpoint_url.as_ref().unwrap()
    }
}

#[derive(Debug)]
pub struct BundleProcessor {
    descriptor: Arc<ProcessBundleDescriptor>,
    creation_ordered_operators: Mutex<Vec<Operator>>,
    topologically_ordered_operators: RwLock<Vec<Operator>>,

    operators: RwLock<HashMap<String, Operator>>,
    receivers: RwLock<HashMap<String, Receiver>>,
    consumers: RwLock<HashMap<String, Vec<String>>>,

    current_bundle_id: Option<String>,
}

impl BundleProcessor {
    pub fn new(descriptor: Arc<ProcessBundleDescriptor>, root_urns: &[&'static str]) -> Arc<Self> {
        let mut consumers: HashMap<String, Vec<String>> = HashMap::new();
        for (transform_id, ptransform) in descriptor.transforms.iter() {
            if is_primitive(ptransform) {
                ptransform
                    .inputs
                    .values()
                    .for_each(|pcollection_id: &String| {
                        if (consumers.get(pcollection_id).is_none()) {
                            consumers.insert(pcollection_id.clone(), Vec::new());
                        }

                        consumers
                            .get_mut(pcollection_id)
                            .unwrap()
                            .push(transform_id.clone());
                    })
            }
        }

        let mut _instance = Arc::new(Self {
            descriptor: descriptor.clone(),
            creation_ordered_operators: Mutex::new(Vec::new()),
            topologically_ordered_operators: RwLock::new(Vec::new()),
            operators: RwLock::new(HashMap::new()),
            receivers: RwLock::new(HashMap::new()),
            consumers: RwLock::new(consumers),
            current_bundle_id: None,
        });

        descriptor
            .transforms
            .iter()
            .for_each(|(transform_id, transform)| {
                if let Some(spec) = &transform.spec {
                    if root_urns.contains(&spec.urn.as_str()) {
                        BundleProcessor::get_operator(_instance.clone(), transform_id.clone());
                    }
                }
            });

        _instance.create_topologically_ordered_operators();

        _instance
    }

    // TODO: the bundle processors are being be passed around by value as parameters
    // to avoid closures with temp references in an async context. However, this
    // should be revisited later.
    fn get_receiver(bundle_processor: Arc<BundleProcessor>, pcollection_id: String) -> Receiver {
        let receivers = bundle_processor.receivers.read().unwrap();

        let receiver = match receivers.get(&pcollection_id) {
            Some(rec) => rec.clone(),
            None => {
                let consumers = bundle_processor.consumers.read().unwrap();

                let pcoll_operators: Vec<Operator> = match consumers.get(&pcollection_id) {
                    Some(v) => {
                        drop(receivers);

                        let mut oprs = Vec::new();
                        for transform_id in v {
                            let cl = bundle_processor.clone();
                            oprs.push(BundleProcessor::get_operator(cl, transform_id.clone()))
                        }
                        oprs
                    }
                    None => Vec::new(),
                };

                let receiver = Receiver::new(pcoll_operators);

                let mut receivers = bundle_processor.receivers.write().unwrap();
                receivers.insert(pcollection_id.clone(), receiver);
                receivers[&pcollection_id].clone()
            }
        };

        receiver
    }

    fn get_operator(bundle_processor: Arc<BundleProcessor>, transform_id: String) -> Operator {
        let get_receiver = |b: Arc<BundleProcessor>, pcollection_id: String| {
            BundleProcessor::get_receiver(b, pcollection_id)
        };

        let operators = bundle_processor.operators.read().unwrap();

        let operator_opt = operators.get(&transform_id);

        if operator_opt.is_some() {
            operator_opt.unwrap().clone()
        } else {
            let descriptor = bundle_processor.descriptor.clone();
            drop(operators);

            let op = create_operator(
                &transform_id,
                OperatorContext {
                    descriptor,
                    get_receiver: Box::new(get_receiver),

                    bundle_processor: bundle_processor.clone(),
                },
            );

            let mut operators = bundle_processor.operators.write().unwrap();
            operators.insert(transform_id.clone(), op);

            let mut creation_ordered_operators =
                bundle_processor.creation_ordered_operators.lock().unwrap();
            creation_ordered_operators.push(operators[&transform_id].clone());

            operators[&transform_id].clone()
        }
    }

    fn create_topologically_ordered_operators(&self) {
        let creation_ordered = self.creation_ordered_operators.lock().unwrap();

        let mut instance_operators = self.topologically_ordered_operators.write().unwrap();

        creation_ordered.iter().rev().for_each(|op: &Operator| {
            instance_operators.push(op.clone());
        });
    }

    pub fn process(&mut self, instruction_id: String) {
        self.current_bundle_id = Some(instruction_id);

        let topologically_ordered_operators = self.topologically_ordered_operators.read().unwrap();
        for o in topologically_ordered_operators.iter().rev() {
            o.start_bundle();
        }
    }
}

// TODO
fn is_primitive(transform: &PTransform) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    use internals::urns;
    use proto::beam::pipeline::FunctionSpec;

    fn make_ptransform(
        urn: &'static str,
        inputs: HashMap<String, String>,
        outputs: HashMap<String, String>,
        payload: Vec<u8>,
    ) -> PTransform {
        PTransform {
            unique_name: "".to_string(),
            spec: Some(FunctionSpec {
                urn: urn.to_string(),
                payload,
            }),
            subtransforms: Vec::with_capacity(0),
            inputs,
            outputs,
            display_data: Vec::with_capacity(0),
            environment_id: "".to_string(),
            annotations: HashMap::with_capacity(0),
        }
    }

    #[test]
    fn test_operator_construction() {
        let descriptor = ProcessBundleDescriptor {
            id: "".to_string(),
            // Note the inverted order should still be resolved correctly
            transforms: HashMap::from([
                (
                    "y".to_string(),
                    make_ptransform(
                        urns::RECORDING_URN,
                        HashMap::from([("input".to_string(), "pc1".to_string())]),
                        HashMap::from([("out".to_string(), "pc2".to_string())]),
                        Vec::with_capacity(0),
                    ),
                ),
                (
                    "z".to_string(),
                    make_ptransform(
                        urns::RECORDING_URN,
                        HashMap::from([("input".to_string(), "pc2".to_string())]),
                        HashMap::with_capacity(0),
                        Vec::with_capacity(0),
                    ),
                ),
                (
                    "x".to_string(),
                    make_ptransform(
                        urns::CREATE_URN,
                        HashMap::with_capacity(0),
                        HashMap::from([("out".to_string(), "pc1".to_string())]),
                        serde_json::to_vec(&["a", "b", "c"]).unwrap(),
                    ),
                ),
            ]),
            pcollections: HashMap::with_capacity(0),
            windowing_strategies: HashMap::with_capacity(0),
            coders: HashMap::with_capacity(0),
            environments: HashMap::with_capacity(0),
            state_api_service_descriptor: None,
            timer_api_service_descriptor: None,
        };

        let processor = BundleProcessor::new(Arc::new(descriptor), &[urns::CREATE_URN]);
    }
}
