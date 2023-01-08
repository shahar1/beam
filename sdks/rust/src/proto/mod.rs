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

// TODO: move this to SDK root?

#![allow(clippy::derive_partial_eq_without_eq, clippy::enum_variant_names)]
pub mod beam_api {
    tonic::include_proto!("beam_api");

    pub use org::apache::beam::model::expansion::v1 as expansion;
    pub use org::apache::beam::model::fn_execution::v1 as fn_execution;
    pub use org::apache::beam::model::interactive::v1 as interactive;
    pub use org::apache::beam::model::job_management::v1 as job_management;
    pub use org::apache::beam::model::pipeline::v1 as pipeline;
}
