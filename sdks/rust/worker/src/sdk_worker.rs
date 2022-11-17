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
use tonic::transport::Channel;

use proto::beam::fn_execution::{
    beam_fn_control_client::BeamFnControlClient, InstructionRequest, InstructionResponse,
    ProcessBundleDescriptor, ProcessBundleResponse, ProcessBundleSplitRequest,
    ProcessBundleSplitResponse,
};

use crate::operators::{create_operator, IOperator, OperatorContext, Receiver};

#[derive(Debug)]
pub struct Worker {
    id: String,
    endpoints: WorkerEndpoints,
    options: HashMap<String, String>,
    control_client: Arc<BeamFnControlClient<Channel>>,
}

impl Worker {
    pub async fn new(id: String, endpoints: WorkerEndpoints) -> Mutex<Self> {
        // TODO: parse URIs in the endpoint struct
        let channel = Channel::builder(endpoints.get_endpoint().parse::<Uri>().unwrap())
            .connect()
            .await
            .expect("Failed to connect to worker");

        Mutex::new(Self {
            id,
            endpoints,
            options: HashMap::new(),
            control_client: Arc::new(BeamFnControlClient::new(channel)),
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
    topologically_ordered_operators: Vec<Box<dyn IOperator>>,
    operators: HashMap<String, Box<dyn IOperator>>,
}
