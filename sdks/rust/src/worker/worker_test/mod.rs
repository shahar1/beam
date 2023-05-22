mod tests {
    use std::{collections::HashMap, sync::Arc};

    use crate::internals::urns;
    use crate::proto::{fn_execution_v1, pipeline_v1};

    use crate::{
        worker::sdk_worker::BundleProcessor,
        worker::test_utils::{reset_log, RECORDING_OPERATOR_LOGS},
    };

    fn make_ptransform(
        urn: &'static str,
        inputs: HashMap<String, String>,
        outputs: HashMap<String, String>,
        payload: Vec<u8>,
    ) -> pipeline_v1::PTransform {
        pipeline_v1::PTransform {
            unique_name: "".to_string(),
            spec: Some(pipeline_v1::FunctionSpec {
                urn: urn.to_string(),
                payload,
            }),
            subtransforms: Vec::with_capacity(0),
            inputs,
            outputs,
            display_data: Vec::with_capacity(0),
            environment_id: "".to_string(),
            annotations: HashMap::with_capacity(0),
        }
    }

    #[tokio::test]
    async fn test_operator_construction() {
        let descriptor = fn_execution_v1::ProcessBundleDescriptor {
            id: "".to_string(),
            // Note the inverted order should still be resolved correctly
            transforms: HashMap::from([
                (
                    "y".to_string(),
                    make_ptransform(
                        urns::RECORDING_URN,
                        HashMap::from([("input".to_string(), "pc1".to_string())]),
                        HashMap::from([("out".to_string(), "pc2".to_string())]),
                        Vec::with_capacity(0),
                    ),
                ),
                (
                    "z".to_string(),
                    make_ptransform(
                        urns::RECORDING_URN,
                        HashMap::from([("input".to_string(), "pc2".to_string())]),
                        HashMap::with_capacity(0),
                        Vec::with_capacity(0),
                    ),
                ),
                (
                    "x".to_string(),
                    make_ptransform(
                        urns::CREATE_URN,
                        HashMap::with_capacity(0),
                        HashMap::from([("out".to_string(), "pc1".to_string())]),
                        serde_json::to_vec(&["a", "b", "c"]).unwrap(),
                    ),
                ),
            ]),
            pcollections: HashMap::with_capacity(0),
            windowing_strategies: HashMap::with_capacity(0),
            coders: HashMap::with_capacity(0),
            environments: HashMap::with_capacity(0),
            state_api_service_descriptor: None,
            timer_api_service_descriptor: None,
        };

        reset_log();

        let processor = BundleProcessor::new(Arc::new(descriptor), &[urns::CREATE_URN]);

        processor.process("bundle_id".to_string()).await;

        let log = RECORDING_OPERATOR_LOGS.lock().unwrap();
        let _log: &Vec<String> = log.as_ref();

        assert_eq!(
            *_log,
            Vec::from([
                "z.start_bundle()",
                "y.start_bundle()",
                "y.process(\"a\")",
                "z.process(\"a\")",
                "y.process(\"b\")",
                "z.process(\"b\")",
                "y.process(\"c\")",
                "z.process(\"c\")",
                "y.finish_bundle()",
                "z.finish_bundle()",
            ])
        );
    }
}
