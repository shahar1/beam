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

// TODO: use relative paths instead of relying on protos copied from Beam root

fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .include_file("mod.rs")
        .compile(
            &[
                "src/beam_protos/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/endpoints.proto",
                "src/beam_protos/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/beam_runner_api.proto",
                "src/beam_protos/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/external_transforms.proto",
                "src/beam_protos/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/metrics.proto",
                "src/beam_protos/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/schema.proto",
                "src/beam_protos/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1/standard_window_fns.proto",
                "src/beam_protos/fn-execution/src/main/proto/org/apache/beam/model/fn_execution/v1/beam_fn_api.proto",
                "src/beam_protos/fn-execution/src/main/proto/org/apache/beam/model/fn_execution/v1/beam_provision_api.proto",
                "src/beam_protos/interactive/src/main/proto/org/apache/beam/model/interactive/v1/beam_interactive_api.proto",
                "src/beam_protos/job-management/src/main/proto/org/apache/beam/model/job_management/v1/beam_artifact_api.proto",
                "src/beam_protos/job-management/src/main/proto/org/apache/beam/model/job_management/v1/beam_expansion_api.proto",
                "src/beam_protos/job-management/src/main/proto/org/apache/beam/model/job_management/v1/beam_job_api.proto",
            ],
            &[
                "src/beam_protos/pipeline/src/main/proto",
                "src/beam_protos/fn-execution/src/main/proto",
                "src/beam_protos/interactive/src/main/proto",
                "src/beam_protos/job-management/src/main/proto",
            ],
        ).unwrap();
}
