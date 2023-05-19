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

use crate::coders::CoderUrnTree;
use crate::internals::pipeline::Pipeline;
use crate::internals::pvalue::{PTransform, PValue};
use crate::internals::urns::IMPULSE_URN;
use crate::proto::pipeline_v1;

pub struct Impulse {
    urn: &'static str,
}

impl Impulse {
    pub fn new() -> Self {
        Self { urn: IMPULSE_URN }
    }
}

impl PTransform<(), Vec<u8>> for Impulse {
    fn expand(
        &self,
        _input: &PValue<()>,
        pipeline: Arc<Pipeline>,
        coder_urn: &CoderUrnTree,
        transform_proto: &mut pipeline_v1::PTransform,
    ) -> PValue<Vec<u8>> {
        let spec = pipeline_v1::FunctionSpec {
            urn: self.urn.to_string(),
            payload: crate::internals::urns::IMPULSE_BUFFER.to_vec(), // Should be able to omit.
        };
        transform_proto.spec = Some(spec);

        pipeline.create_pcollection_internal(coder_urn, pipeline.clone())
    }
}

impl Default for Impulse {
    fn default() -> Self {
        Self::new()
    }
}
