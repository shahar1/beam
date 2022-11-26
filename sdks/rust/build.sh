#!/bin/bash

#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

# TODO: make build.rs use relative paths and stop copying the protos from Beam
# root

cd proto
rm -r beam_protos/
cp -r ../../../model/ beam_protos
cd ..

cargo test --workspace --exclude proto
# TODO: set build to fail on warnings when code is ready
# cargo clippy --all-targets --all-features -- -D warnings
cargo clippy
cargo fmt --all -- --check
