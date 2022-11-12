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
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::net;
use tokio_util::sync::CancellationToken;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::sdk_worker::{Worker, WorkerEndpoints};
use beam_fn_external_worker_pool_server::{
    BeamFnExternalWorkerPool, BeamFnExternalWorkerPoolServer,
};
use proto::beam::fn_execution::{
    beam_fn_external_worker_pool_server, StartWorkerRequest, StartWorkerResponse,
    StopWorkerRequest, StopWorkerResponse,
};

#[derive(Debug)]
struct BeamFnExternalWorkerPoolService {
    workers: Arc<Mutex<HashMap<String, Worker>>>,
}

#[tonic::async_trait]
impl BeamFnExternalWorkerPool for BeamFnExternalWorkerPoolService {
    async fn start_worker(
        &self,
        request: Request<StartWorkerRequest>,
    ) -> Result<Response<StartWorkerResponse>, Status> {
        let mut _workers = self.workers.lock().unwrap();

        let req = &request.get_ref();
        let control_endpoint_url = req.control_endpoint.as_ref().map(|t| t.url.clone());

        // TODO: review cloning
        _workers.insert(
            req.worker_id.clone(),
            Worker::new(
                req.worker_id.clone(),
                WorkerEndpoints::new(control_endpoint_url),
            ),
        );

        Ok(Response::new(StartWorkerResponse::default()))
    }

    async fn stop_worker(
        &self,
        request: Request<StopWorkerRequest>,
    ) -> Result<Response<StopWorkerResponse>, Status> {
        let mut _workers = self.workers.lock().unwrap();

        let req = &request.get_ref();
        let worker_id = &req.worker_id.to_owned();

        if let Some(w) = _workers.get_mut(worker_id) {
            w.stop();
            _workers.remove(worker_id);
        };

        Ok(Response::new(StopWorkerResponse::default()))
    }
}

pub struct ExternalWorkerPool {
    address: SocketAddr,
    cancellation_token: CancellationToken,
}

impl ExternalWorkerPool {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            cancellation_token: CancellationToken::new(),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: add logging
        println!("Starting loopback workers at {}", self.address);

        let svc = BeamFnExternalWorkerPoolServer::new(BeamFnExternalWorkerPoolService {
            workers: Arc::new(Mutex::new(HashMap::new())),
        });

        Server::builder()
            .add_service(svc)
            .serve_with_shutdown(self.address, self.cancellation_token.cancelled())
            .await?;

        Ok(())
    }

    // TODO: implement timeout for graceful shutdown
    pub async fn stop(&self, timeout: Duration) {
        // TODO: add logging
        println!("Shutting down external workers.");

        self.cancellation_token.cancel();
    }
}
