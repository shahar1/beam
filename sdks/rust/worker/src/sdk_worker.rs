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
use std::sync::{Arc, Mutex};

use http::Uri;
use proto::beam::pipeline::PTransform;
use tonic::transport::Channel;

use proto::beam::fn_execution::{
    beam_fn_control_client::BeamFnControlClient, InstructionRequest, InstructionResponse,
    ProcessBundleDescriptor, ProcessBundleResponse, ProcessBundleSplitRequest,
    ProcessBundleSplitResponse,
};

use crate::operators::{create_operator, Operator, OperatorContext, Receiver};

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
    pub async fn new(id: String, endpoints: WorkerEndpoints) -> Mutex<Self> {
        // TODO: parse URIs in the endpoint struct
        let channel = Channel::builder(endpoints.get_endpoint().parse::<Uri>().unwrap())
            .connect()
            .await
            .expect("Failed to connect to worker");

        Mutex::new(Self {
            control_client: Arc::new(BeamFnControlClient::new(channel)),

            process_bundle_descriptors: HashMap::new(),
            bundle_processors: HashMap::new(),
            active_bundle_processors: HashMap::new(),
            id,
            endpoints,
            options: HashMap::new(),
        })
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
    descriptor: ProcessBundleDescriptor,

    creation_ordered_operators: Mutex<Vec<Operator>>,
    topologically_ordered_operators: Vec<Operator>,
    operators: Mutex<HashMap<String, Operator>>,
    receivers: Mutex<HashMap<String, Receiver>>,
    consumers: Mutex<HashMap<String, Vec<String>>>,

    current_bundle_id: Option<String>,
}

impl BundleProcessor {
    pub fn new(descriptor: ProcessBundleDescriptor, root_urns: &[&'static str]) -> Self {
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

        let mut _instance = Self {
            descriptor: descriptor.clone(),
            creation_ordered_operators: Mutex::new(Vec::new()),
            topologically_ordered_operators: Vec::new(),
            operators: Mutex::new(HashMap::new()),
            receivers: Mutex::new(HashMap::new()),
            consumers: Mutex::new(consumers),
            current_bundle_id: None,
        };

        descriptor
            .transforms
            .iter()
            .for_each(|(transform_id, transform)| {
                if let Some(spec) = &transform.spec {
                    if root_urns.contains(&spec.urn.as_str()) {
                        _instance.get_operator(transform_id.clone());
                    }
                }
            });

        _instance.create_topologically_ordered_operators();

        _instance
    }

    pub fn get_receiver(&self, pcollection_id: String) -> Receiver {
        let mut receivers = self.receivers.lock().unwrap();
        let consumers = self.consumers.lock().unwrap();

        receivers.entry(pcollection_id.clone()).or_insert_with(|| {
            let pcoll_operators: Vec<Operator> = match consumers.get(&pcollection_id) {
                Some(v) => {
                    let mut oprs = Vec::new();
                    for transform_id in v {
                        oprs.push(self.get_operator(transform_id.clone()))
                    }
                    oprs
                }
                None => Vec::new(),
            };

            Receiver::new(pcoll_operators)
        });
        receivers[&pcollection_id].clone()
    }

    pub fn get_operator(&self, transform_id: String) -> Operator {
        let mut operators = self.operators.lock().unwrap();
        let get_receiver = |pcollection_id: String| self.get_receiver(pcollection_id);

        operators.entry(transform_id.clone()).or_insert_with(|| {
            create_operator(
                &transform_id,
                OperatorContext {
                    descriptor: &self.descriptor,
                    get_receiver: Box::new(get_receiver),
                },
            )
        });

        let mut creation_ordered_operators = self.creation_ordered_operators.lock().unwrap();
        creation_ordered_operators.push(operators[&transform_id].clone());

        operators[&transform_id].clone()
    }

    fn create_topologically_ordered_operators(&mut self) {
        let creation_ordered = self.creation_ordered_operators.lock().unwrap().clone();
        self.topologically_ordered_operators = creation_ordered.into_iter().rev().collect();
    }
}

// TODO
fn is_primitive(transform: &PTransform) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_construction() {}
}
