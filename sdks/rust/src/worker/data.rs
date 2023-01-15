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

use std::sync::Arc;

use async_cell::sync::AsyncCell;
use dashmap::DashMap;
use itertools::Itertools;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

use super::interceptors::WorkerIdInterceptor;
use crate::proto::{
    fn_execution_v1, fn_execution_v1::beam_fn_data_client as beam_fn_data_client_v1,
};

type ArcAsyncCellSenderElements = Arc<AsyncCell<mpsc::UnboundedSender<fn_execution_v1::Elements>>>;
type InstructionId = String;

#[derive(Debug)]
pub struct MultiplexingDataChannel {
    client:
        beam_fn_data_client_v1::BeamFnDataClient<InterceptedService<Channel, WorkerIdInterceptor>>,
    rx: Arc<Mutex<mpsc::UnboundedReceiver<fn_execution_v1::Elements>>>,
    tx: mpsc::UnboundedSender<fn_execution_v1::Elements>,
    consumers: DashMap<InstructionId, ArcAsyncCellSenderElements>,
}

impl MultiplexingDataChannel {
    pub fn try_new(id: String, data_endpoint: String) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(data_endpoint)?.connect_lazy();
        let client = beam_fn_data_client_v1::BeamFnDataClient::with_interceptor(
            channel,
            WorkerIdInterceptor::new(id),
        );
        let (tx, rx) = mpsc::unbounded_channel();

        Ok(Self {
            client,
            tx,
            rx: Arc::new(Mutex::new(rx)),
            consumers: DashMap::new(),
        })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let rx = self.rx.clone();
        let outbound = async_stream::stream! {
            while let Some(data_res) = rx.lock().await.recv().await {
                yield data_res
            }
        };
        let response = self.client.data(outbound).await?;
        let mut inbound = response.into_inner();

        while let Some(mut elements) = inbound.message().await? {
            // Sort (stable), group and merge Data and Timers by instruction_id into new Elements.
            elements
                .data
                .sort_by(|d1, d2| d1.instruction_id.cmp(&d2.instruction_id));
            elements
                .timers
                .sort_by(|t1, t2| t1.instruction_id.cmp(&t2.instruction_id));

            let gd = elements
                .data
                .into_iter()
                .group_by(|d| d.instruction_id.clone());
            let gt = elements
                .timers
                .into_iter()
                .group_by(|t| t.instruction_id.clone());

            // Result has to be materialized because group_by is not Sync.
            let elements = gd
                .into_iter()
                .merge_join_by(gt.into_iter(), |(kgd, _), (kgt, _)| kgd.cmp(kgt))
                .map(|eob| match eob {
                    itertools::EitherOrBoth::Both((kd, vd), (_, vt)) => (
                        kd,
                        fn_execution_v1::Elements {
                            data: vd.collect_vec(),
                            timers: vt.collect_vec(),
                        },
                    ),
                    itertools::EitherOrBoth::Left((kd, vd)) => (
                        kd,
                        fn_execution_v1::Elements {
                            data: vd.collect_vec(),
                            ..Default::default()
                        },
                    ),
                    itertools::EitherOrBoth::Right((kt, vt)) => (
                        kt,
                        fn_execution_v1::Elements {
                            timers: vt.collect_vec(),
                            ..Default::default()
                        },
                    ),
                })
                .collect_vec();

            // Iterate over Elements and send to the consumer for instruction_id.
            for (id, es) in elements {
                // TODO(sjvanrossum): This reflects the Java SDK implementation, but may be improved on if:
                // - Senders are cached by (instruction_id, transform_id) in a HashMap and discarded on is_last.
                // - DashMap is replaced by a moka::sync::Cache configured as a simple concurrent hash map.
                //
                // Benchmarking required to show if this is beneficial over some additional complexity.
                // Repeatedly accessing the DashMap should be fine for now.
                let cell = self
                    .consumers
                    .entry(id.clone())
                    .or_insert(AsyncCell::shared())
                    .value()
                    .clone();

                _ = cell.get().await.send(es);
            }
        }

        Ok(())
    }

    pub async fn stop(&self) {
        self.rx.lock().await.close()
    }

    pub fn register_consumer(
        &self,
        consumer_id: InstructionId,
        tx: mpsc::UnboundedSender<fn_execution_v1::Elements>,
    ) {
        self.consumers
            .entry(consumer_id)
            .or_insert(Arc::new(AsyncCell::new_with(tx)));
    }

    pub fn unregister_consumer(&self, consumer_id: &InstructionId) {
        self.consumers.remove(consumer_id);
    }

    #[inline]
    pub fn get_producer(&self) -> mpsc::UnboundedSender<fn_execution_v1::Elements> {
        self.tx.clone()
    }
}
