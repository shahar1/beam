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

use std::{sync::Arc, sync::Mutex};

use proto::beam::{fn_execution::ProcessBundleDescriptor, pipeline as proto_pipeline};
use worker::sdk_worker::BundleProcessor;

use crate::runner::{RunnerI, Task};

pub struct DirectRunner;

impl RunnerI for DirectRunner {
    fn run_pipeline(&self, pipeline: Arc<Mutex<proto_pipeline::Pipeline>>) -> Task {
        let p = pipeline.lock().unwrap();

        // TODO: use this to define the descriptor instead of p
        // let proto = rewrite_side_inputs(pipeline, state_cache_ref);

        // TODO: review cloning
        let descriptor = ProcessBundleDescriptor {
            id: "".to_string(),
            transforms: p
                .components
                .as_ref()
                .expect("Missing components")
                .transforms
                .clone(),
            pcollections: p
                .components
                .as_ref()
                .expect("Missing PCollections")
                .pcollections
                .clone(),
            windowing_strategies: p
                .components
                .as_ref()
                .expect("Missing windowing strategies")
                .windowing_strategies
                .clone(),
            coders: p
                .components
                .as_ref()
                .expect("Missing coders")
                .coders
                .clone(),
            environments: p
                .components
                .as_ref()
                .expect("Missing environments")
                .environments
                .clone(),
            state_api_service_descriptor: None,
            timer_api_service_descriptor: None,
        };

        drop(p);

        let processor = BundleProcessor::new(Arc::new(descriptor), &[crate::IMPULSE_URN]);

        // TODO: this needs to be an async method
        processor.process("bundle_id".to_string());

        unimplemented!()
    }
}
