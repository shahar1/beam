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

//TODO: organize tests in a better way
#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::sync::Arc;

    use beam_core::pvalue::{PType, PValue};
    use coders::required_coders::BytesCoder;
    use internals::pipeline::Pipeline;
    use runners::direct_runner::DirectRunner;
    use runners::runner::RunnerI;
    use transforms::impulse::Impulse;

    #[tokio::test]
    async fn run_direct_runner() {
        let runner = DirectRunner::new();

        runner.run(|root| root.apply(Impulse::new())).await;
    }

    #[tokio::test]
    async fn run_impulse_expansion() {
        let p = Arc::new(Pipeline::new());
        let root = PValue::new_root(p.clone());

        let pcoll = root.apply(Impulse::new());

        // TODO: test proto coders
        // let pipeline_proto = runner.pipeline.proto.lock().unwrap();
        // let proto_coders = pipeline_proto.components.unwrap().coders;
        // let coder = *proto_coders
        //     .get(&root_clone.pcoll_proto.coder_id)
        //     .unwrap();

        let bytes_coder_type_id = BytesCoder::new().type_id();
        let coder = p.get_coder::<BytesCoder, Vec<u8>>(&bytes_coder_type_id);

        assert_eq!(*pcoll.get_type(), PType::PCollection);
        assert_eq!(coder.type_id(), bytes_coder_type_id);
    }
}
