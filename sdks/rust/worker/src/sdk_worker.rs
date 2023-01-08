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
use std::sync::{Arc, Mutex, RwLock};

use http::Uri;
use proto::beam_api::pipeline::PTransform;
use tonic::transport::Channel;

use proto::beam_api::fn_execution::{
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
    creation_ordered_operators: Mutex<Vec<Arc<Operator>>>,
    topologically_ordered_operators: RwLock<Vec<Arc<Operator>>>,

    operators: RwLock<HashMap<String, Arc<Operator>>>,
    receivers: RwLock<HashMap<String, Arc<Receiver>>>,
    consumers: RwLock<HashMap<String, Vec<String>>>,

    current_bundle_id: Mutex<Option<String>>,
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
                        if consumers.get(pcollection_id).is_none() {
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
            current_bundle_id: Mutex::new(None),
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
    fn get_receiver(
        bundle_processor: Arc<BundleProcessor>,
        pcollection_id: String,
    ) -> Arc<Receiver> {
        let receivers = bundle_processor.receivers.read().unwrap();

        let receiver = match receivers.get(&pcollection_id) {
            Some(rec) => rec.clone(),
            None => {
                let consumers = bundle_processor.consumers.read().unwrap();

                let pcoll_operators: Vec<Arc<Operator>> = match consumers.get(&pcollection_id) {
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
                receivers.insert(pcollection_id.clone(), Arc::new(receiver));
                receivers[&pcollection_id].clone()
            }
        };

        receiver
    }

    fn get_operator(bundle_processor: Arc<BundleProcessor>, transform_id: String) -> Arc<Operator> {
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
                Arc::new(OperatorContext {
                    descriptor,
                    get_receiver: Box::new(get_receiver),

                    bundle_processor: bundle_processor.clone(),
                }),
            );

            let mut operators = bundle_processor.operators.write().unwrap();
            operators.insert(transform_id.clone(), Arc::new(op));

            let mut creation_ordered_operators =
                bundle_processor.creation_ordered_operators.lock().unwrap();
            creation_ordered_operators.push(operators[&transform_id].clone());

            operators[&transform_id].clone()
        }
    }

    fn create_topologically_ordered_operators(&self) {
        let creation_ordered = self.creation_ordered_operators.lock().unwrap();

        let mut instance_operators = self.topologically_ordered_operators.write().unwrap();

        creation_ordered
            .iter()
            .rev()
            .for_each(|op: &Arc<Operator>| {
                instance_operators.push(op.clone());
            });
    }

    pub async fn process(&self, instruction_id: String) {
        let mut current_bundle_id = self.current_bundle_id.lock().unwrap();
        current_bundle_id.replace(instruction_id);
        drop(current_bundle_id);

        let topologically_ordered_operators = self.topologically_ordered_operators.read().unwrap();
        for o in topologically_ordered_operators.iter().rev() {
            o.start_bundle();
        }

        for o in topologically_ordered_operators.iter() {
            o.finish_bundle();
        }
    }
}

// TODO
fn is_primitive(transform: &PTransform) -> bool {
    true
}
