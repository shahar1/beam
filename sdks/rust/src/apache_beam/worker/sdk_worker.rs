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

#[derive(Debug)]
pub struct Worker {
  id: String,
  endpoints: WorkerEndpoints,
  // TODO: review placeholder
  options: HashMap<String, String>,
}

impl Worker {
  pub fn new(id: String, endpoints: WorkerEndpoints) -> Self {
    Self {
      id,
      endpoints,
      options: HashMap::new(),
    }
  }

  pub fn stop(&mut self) {
    unimplemented!()
  }
}

#[derive(Debug)]
pub struct WorkerEndpoints {
  control_endpoint_url: Option<String>,
}

impl WorkerEndpoints {
  pub fn new(control_endpoint_url: Option<String>) -> Self {
    Self {
      control_endpoint_url,
    }
  }
}
