// Licensed to the Apache Software Foundation (ASF) under one or more
// contributor license agreements.  See the NOTICE file distributed with
// this work for additional information regarding copyright ownership.
// The ASF licenses this file to You under the Apache License, Version 2.0
// (the "License"); you may not use this file except in compliance with
// the License.  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package main

import (
	"context"
	"errors"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/apache/beam/sdks/v2/go/pkg/beam/artifact"

	fnpb "github.com/apache/beam/sdks/v2/go/pkg/beam/model/fnexecution_v1"
	pipepb "github.com/apache/beam/sdks/v2/go/pkg/beam/model/pipeline_v1"
	"github.com/apache/beam/sdks/v2/go/pkg/beam/provision"
	"github.com/apache/beam/sdks/v2/go/pkg/beam/util/execx"
	"github.com/apache/beam/sdks/v2/go/pkg/beam/util/grpcx"
)

// TODO(sjvanrossum): Move to artifact package once this SDK matures.
const URNRustWorkerBinaryRole = "beam:artifact:role:rust_worker_binary:v1"

var (
	// Contract: https://s.apache.org/beam-fn-api-container-contract.

	id                = flag.String("id", "", "Local identifier (required).")
	loggingEndpoint   = flag.String("logging_endpoint", "", "Local logging endpoint for FnHarness (required).")
	artifactEndpoint  = flag.String("artifact_endpoint", "", "Local artifact endpoint for FnHarness (required).")
	provisionEndpoint = flag.String("provision_endpoint", "", "Local provision endpoint for FnHarness (required).")
	controlEndpoint   = flag.String("control_endpoint", "", "Local control endpoint for FnHarness (required).")
	semiPersistDir    = flag.String("semi_persist_dir", "/tmp", "Local semi-persistent directory (optional).")
)

func main() {
	flag.Parse()
	if *id == "" {
		log.Fatal("No id provided.")
	}
	if *provisionEndpoint == "" {
		log.Fatal("No provision endpoint provided.")
	}

	ctx := grpcx.WriteWorkerID(context.Background(), *id)

	info, err := provision.Info(ctx, *provisionEndpoint)
	if err != nil {
		log.Fatalf("Failed to obtain provisioning information: %v", err)
	}
	log.Printf("Provision info:\n%v", info)

	err = ensureEndpointsSet(info)
	if err != nil {
		log.Fatalf("Endpoint not set: %v", err)
	}
	log.Printf("Initializing Rust harness: %v", strings.Join(os.Args, " "))

	// (1) Obtain the pipeline options

	options, err := provision.ProtoToJSON(info.GetPipelineOptions())
	if err != nil {
		log.Fatalf("Failed to convert pipeline options: %v", err)
	}

	// (2) Retrieve the staged files.
	//
	// The Rust SDK harness downloads the worker binary and invokes
	// it.

	dir := filepath.Join(*semiPersistDir, "staged")
	artifacts, err := artifact.Materialize(ctx, *artifactEndpoint, info.GetDependencies(), info.GetRetrievalToken(), dir)
	if err != nil {
		log.Fatalf("Failed to retrieve staged files: %v", err)
	}

	name, err := getRustWorkerArtifactName(artifacts)
	if err != nil {
		log.Fatalf("Failed to get Rust Worker Artifact Name: %v", err)
	}

	// (3) The persist dir may be on a noexec volume, so we must
	// copy the binary to a different location to execute.
	const prog = "/bin/worker"
	if err := copyExe(filepath.Join(dir, name), prog); err != nil {
		log.Fatalf("Failed to copy worker binary: %v", err)
	}

	args := []string{
		"worker",
		"--id=" + *id,
		"--control_endpoint=" + *controlEndpoint,
		"--logging_endpoint=" + *loggingEndpoint,
		"--options=" + options,
	}
	if info.GetStatusEndpoint() != nil {
		args = append(args, "--status-endpoint="+info.GetStatusEndpoint().GetUrl())
	}

	if len(info.GetRunnerCapabilities()) > 0 {
		args = append(args, "--runner-capabilities="+strings.Join(info.GetRunnerCapabilities(), " "))
	}

	log.Fatalf("User program exited: %v", execx.Execute(prog, args...))
}

func getRustWorkerArtifactName(artifacts []*pipepb.ArtifactInformation) (string, error) {
	if len(artifacts) == 0 {
		return "", errors.New("no artifacts staged")
	}

	for _, a := range artifacts {
		if a.RoleUrn == "beam:artifact:role:rust_worker_binary:v1" {
			name, _ := artifact.MustExtractFilePayload(a)
			return name, nil
		}
	}
	return "", fmt.Errorf("no artifact named '%v' found", URNRustWorkerBinaryRole)
}

func ensureEndpointsSet(info *fnpb.ProvisionInfo) error {
	// TODO(BEAM-8201): Simplify once flags are no longer used.
	if info.GetLoggingEndpoint().GetUrl() != "" {
		*loggingEndpoint = info.GetLoggingEndpoint().GetUrl()
	}
	if info.GetArtifactEndpoint().GetUrl() != "" {
		*artifactEndpoint = info.GetArtifactEndpoint().GetUrl()
	}
	if info.GetControlEndpoint().GetUrl() != "" {
		*controlEndpoint = info.GetControlEndpoint().GetUrl()
	}

	if *loggingEndpoint == "" {
		return errors.New("no logging endpoint provided")
	}
	if *artifactEndpoint == "" {
		return errors.New("no artifact endpoint provided")
	}
	if *controlEndpoint == "" {
		return errors.New("no control endpoint provided")
	}

	return nil
}

func copyExe(from, to string) error {
	src, err := os.Open(from)
	if err != nil {
		return err
	}
	defer src.Close()

	dst, err := os.OpenFile(to, os.O_WRONLY|os.O_CREATE, 0755)
	if err != nil {
		return err
	}

	if _, err := io.Copy(dst, src); err != nil {
		return err
	}
	return dst.Close()
}
