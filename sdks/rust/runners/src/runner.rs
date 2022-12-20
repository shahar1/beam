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

use futures::future::Future;
use std::{collections::HashMap, pin::Pin, sync::Arc, sync::Mutex};

use async_trait::async_trait;

use beam_core::pvalue::{get_pcollection_name, PType, PValue, Pipeline};
use coders::standard_coders::BytesCoder;
use proto::beam::pipeline as proto_pipeline;

pub type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

// TODO: implement PipelineResult

/// A Runner is the object that takes a pipeline definition and actually
/// executes, e.g. locally or on a distributed system.
#[async_trait]
pub trait RunnerI {
    /// Runs the transform.
    /// Resolves to an instance of PipelineResult when the pipeline completes.
    /// Use run_async() to execute the pipeline in the background.
    async fn run<F>(&self, pipeline: F)
    where
        F: FnOnce(PValue) -> Task + Send,
    {
        self.run_async(pipeline).await;
    }

    /// run_async() is the asynchronous version of run(), does not wait until
    /// pipeline finishes. Use the returned PipelineResult to query job
    /// status.
    async fn run_async<F>(&self, pipeline: F)
    where
        F: FnOnce(PValue) -> Task + Send,
    {
        let p = Arc::new(Pipeline::new());
        let root = PValue::new_root(p.clone());

        (pipeline)(root).await;
        self.run_pipeline(p.get_proto());
    }

    fn run_pipeline(&self, pipeline: Arc<Mutex<proto_pipeline::Pipeline>>) -> Task;
}

// TODO: remove this temporary runner
pub struct PlaceholderRunner {
    pipeline: Arc<Pipeline>,
}

impl PlaceholderRunner {
    pub fn new() -> Self {
        Self {
            pipeline: Arc::new(Pipeline::new()),
        }
    }

    pub fn get_pipeline_arc(&self) -> Arc<Pipeline> {
        self.pipeline.clone()
    }

    pub fn run<'a>(&'a self) -> PValue {
        let pcoll_name = get_pcollection_name();

        let proto_coder_id = self.pipeline.register_coder_proto(proto_pipeline::Coder {
            spec: Some(proto_pipeline::FunctionSpec {
                urn: String::from(coders::standard_coders::BYTES_CODER_URN),
                payload: Vec::with_capacity(0),
            }),
            component_coder_ids: Vec::with_capacity(0),
        });

        self.pipeline
            .register_coder::<BytesCoder, Vec<u8>>(Box::new(BytesCoder::new()));

        let output_proto = proto_pipeline::PCollection {
            unique_name: pcoll_name.clone(),
            coder_id: proto_coder_id,
            is_bounded: proto_pipeline::is_bounded::Enum::Bounded as i32,
            windowing_strategy_id: "placeholder".to_string(),
            display_data: Vec::with_capacity(0),
        };

        let impulse_proto = proto_pipeline::PTransform {
            unique_name: "root".to_string(),
            spec: None,
            subtransforms: Vec::with_capacity(0),
            inputs: HashMap::with_capacity(0),
            outputs: HashMap::from([("out".to_string(), pcoll_name.clone())]),
            display_data: Vec::with_capacity(0),
            environment_id: "".to_string(),
            annotations: HashMap::with_capacity(0),
        };

        self.pipeline.register_proto_transform(impulse_proto);

        PValue::new(PType::Root, pcoll_name, output_proto, self.pipeline.clone())
    }
}

impl Default for PlaceholderRunner {
    fn default() -> Self {
        Self::new()
    }
}
