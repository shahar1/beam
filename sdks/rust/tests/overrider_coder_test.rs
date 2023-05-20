use apache_beam::{
    coders::{
        required_coders::BytesCoder,
        urns::{BYTES_CODER_URN, UNIT_CODER_URN, VARINT_CODER_URN},
    },
    runners::{direct_runner::DirectRunner, runner::RunnerI},
    transforms::create::Create,
};

#[tokio::test]
async fn test_root_coder() {
    DirectRunner::new()
        .run(|root| {
            assert_eq!(root.coder_urn(), UNIT_CODER_URN);
            root
        })
        .await;
}

#[tokio::test]
async fn test_default_coder() {
    DirectRunner::new()
        .run(|root| {
            let a = root.apply(Create::new(vec![1, 2, 3]));
            assert_eq!(a.coder_urn(), VARINT_CODER_URN);
            a
        })
        .await;
}

#[tokio::test]
async fn test_override_coder() {
    DirectRunner::new()
        .run(|root| {
            let a = root.apply_with_coder::<BytesCoder, _, _>(Create::new(vec![1, 2, 3]));
            assert_eq!(a.coder_urn(), BYTES_CODER_URN);
            a
        })
        .await;
}
