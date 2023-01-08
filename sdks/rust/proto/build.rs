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

// TODO: compile protos in proto/src instead of the target directory for better
// organization and to enable version control?
fn main() {
    let pipeline_root = "../../../model/pipeline/src/main/proto";
    let pipeline_dir = "../../../model/pipeline/src/main/proto/org/apache/beam/model/pipeline/v1";

    let fn_exec_root = "../../../model/fn-execution/src/main/proto";
    let fn_exec_dir =
        "../../../model/fn-execution/src/main/proto/org/apache/beam/model/fn_execution/v1";

    let job_root = "../../../model/job-management/src/main/proto";
    let job_dir =
        "../../../model/job-management/src/main/proto/org/apache/beam/model/job_management/v1";

    let interactive_root = "../../../model/interactive/src/main/proto";
    let interactive_dir =
        "../../../model/interactive/src/main/proto/org/apache/beam/model/interactive/v1";

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .include_file("protos.rs")
        .compile(
            &[
                format!("{}/{}", pipeline_dir, "beam_runner_api.proto"),
                format!("{}/{}", pipeline_dir, "endpoints.proto"),
                format!("{}/{}", pipeline_dir, "external_transforms.proto"),
                format!("{}/{}", pipeline_dir, "metrics.proto"),
                format!("{}/{}", pipeline_dir, "schema.proto"),
                format!("{}/{}", pipeline_dir, "standard_window_fns.proto"),
                format!("{}/{}", fn_exec_dir, "beam_fn_api.proto"),
                format!("{}/{}", fn_exec_dir, "beam_provision_api.proto"),
                format!("{}/{}", job_dir, "beam_artifact_api.proto"),
                format!("{}/{}", job_dir, "beam_expansion_api.proto"),
                format!("{}/{}", job_dir, "beam_job_api.proto"),
                format!("{}/{}", interactive_dir, "beam_interactive_api.proto"),
            ],
            &[pipeline_root, fn_exec_root, job_root, interactive_root],
        )
        .unwrap_or_else(|e| panic!("Protobuf compile error: {:?}", e));
}
