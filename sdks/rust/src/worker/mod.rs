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

mod anonymous_coder;
pub(in crate::worker) use anonymous_coder::AnonymousCoder;

mod external_worker_service;
pub mod operators;

pub use external_worker_service::ExternalWorkerPool;
pub use operators::Receiver;

pub mod sdk_worker;
pub mod worker_main;

// TODO: organize this in a better way
pub mod test_utils {
    use std::sync::Mutex;

    pub static RECORDING_OPERATOR_LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

    pub fn reset_log() {
        let mut log = RECORDING_OPERATOR_LOGS.lock().unwrap();
        *log.as_mut() = Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        coders::{Coder, Context},
        elem_types::ElemType,
        worker::anonymous_coder::AnonymousCoder,
    };
    use serde::{Deserialize, Serialize};

    #[test]
    fn serde_custom_coder() {
        #[derive(Clone, PartialEq, Eq, Debug)]
        struct MyElement {
            some_field: String,
        }

        impl ElemType for MyElement {}

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        struct MyCoder;

        impl Coder for MyCoder {
            type E = MyElement;

            fn get_coder_urn() -> &'static str
            where
                Self: Sized,
            {
                "beam:dofn:rustsdk:1.0:MyCoder" // TODO auto-gen via #[derive(Coder)]
            }

            fn encode(
                &self,
                element: Self::E,
                writer: &mut dyn std::io::Write,
                _context: &Context,
            ) -> Result<usize, std::io::Error> {
                writer
                    .write_all(format!("ENCPREFIX{}", element.some_field).as_bytes())
                    .map(|_| 0) // TODO make Result<usize, std::io::Error> to Result<(), std::io::Error>
            }

            fn decode(
                &self,
                reader: &mut dyn std::io::Read,
                _context: &Context,
            ) -> Result<Self::E, std::io::Error> {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;

                let encoded_element = String::from_utf8(buf).unwrap();
                let element = encoded_element.strip_prefix("ENCPREFIX").unwrap();
                Ok(MyElement {
                    some_field: element.to_string(),
                })
            }
        }

        let my_coder = MyCoder::default();

        let coder_proto = my_coder.to_proto(vec![]);
        // serialize `proto_coder` into binary format, send to runners and then to SDK harness, then deserialize back to `proto_coder` agin.
        let anon_coder = AnonymousCoder::from(coder_proto);

        let element = MyElement {
            some_field: "some_value".to_string(),
        };

        let mut encoded_element = vec![];
        anon_coder
            .encode(&element, &mut encoded_element, &Context::WholeStream)
            .unwrap();

        let decoded_element: MyElement = anon_coder
            .decode(&mut encoded_element.as_slice(), &Context::WholeStream)
            .unwrap();

        assert_eq!(decoded_element, element);
    }
}
