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
use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;

use crate::elem_types::ElemType;
use crate::internals::pvalue::PValue;
use crate::proto::pipeline_v1;

pub type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

// TODO: implement PipelineResult

/// A Runner is the object that takes a pipeline definition and actually
/// executes, e.g. locally or on a distributed system.
#[async_trait]
pub trait RunnerI {
    fn new() -> Self;

    /// Runs the transform.
    /// Resolves to an instance of PipelineResult when the pipeline completes.
    /// Use run_async() to execute the pipeline in the background.
    async fn run<Out, F>(&self, pipeline: F)
    where
        Out: ElemType,
        F: FnOnce(PValue<()>) -> PValue<Out> + Send, // TODO: Don't require a return value.
    {
        self.run_async(pipeline).await;
    }

    /// run_async() is the asynchronous version of run(), does not wait until
    /// pipeline finishes. Use the returned PipelineResult to query job
    /// status.
    async fn run_async<Out, F>(&self, pipeline: F)
    where
        Out: ElemType,
        F: FnOnce(PValue<()>) -> PValue<Out> + Send,
    {
        let root = PValue::<()>::root();
        let inner_pipeline = root.get_pipeline_arc();

        (pipeline)(root); // pipeline construction, affecting root's inner pipeline object

        self.run_pipeline(inner_pipeline.get_proto()).await;
    }

    async fn run_pipeline(&self, pipeline: Arc<std::sync::Mutex<pipeline_v1::Pipeline>>);
}
