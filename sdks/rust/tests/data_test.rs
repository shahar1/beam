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

use async_cell::sync::AsyncCell;
use futures::stream::StreamExt;
use lazy_static::lazy_static;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{
    transport::{Server},
    Status,
};

use apache_beam::proto::fn_execution::v1::{
    beam_fn_data_server::BeamFnDataServer,
    elements::{Data, Timers},
    Elements,
};

// Contains two mock BeamFnData servers, one to consume Elements and one to produce Elements.
mod mock {
    use std::pin::Pin;
    use std::sync::Arc;

    use async_cell::sync::AsyncCell;
    use futures::{stream, Stream, StreamExt};
    use tokio::sync::Mutex;
    use tonic::{Request, Response, Status, Streaming};

    use apache_beam::proto::fn_execution::v1::{beam_fn_data_server::BeamFnData, Elements};

    use crate::ELEMENTS;

    pub struct DataProducer {}

    #[tonic::async_trait]
    impl BeamFnData for DataProducer {
        type DataStream =
            Pin<Box<dyn Stream<Item = Result<Elements, tonic::Status>> + Send + 'static>>;

        async fn data(
            &self,
            request: Request<Streaming<Elements>>,
        ) -> Result<Response<Self::DataStream>, Status> {
            Ok(Response::new(Box::pin(stream::iter(
                ELEMENTS.clone().into_iter().map(Ok),
            ))))
        }
    }

    pub struct DataConsumer {
        pub stream: Arc<AsyncCell<Streaming<Elements>>>,
    }

    #[tonic::async_trait]
    impl BeamFnData for DataConsumer {
        type DataStream =
            Pin<Box<dyn Stream<Item = Result<Elements, tonic::Status>> + Send + 'static>>;

        async fn data(
            &self,
            request: Request<Streaming<Elements>>,
        ) -> Result<Response<Self::DataStream>, Status> {
            self.stream.set(request.into_inner());
            Ok(Response::new(Box::pin(stream::empty())))
        }
    }
}

lazy_static! {
    static ref ELEMENTS: Vec<Elements> = vec![
        Elements {
            data: (0..=255)
                .map(|x| Data {
                    instruction_id: "foo".to_owned(),
                    data: vec![x],
                    ..Default::default()
                })
                .chain((0..=255).map(|x| Data {
                    instruction_id: "bar".to_owned(),
                    data: vec![x],
                    ..Default::default()
                }))
                .collect::<Vec<_>>(),
            timers: (0..=255)
                .map(|x| Timers {
                    instruction_id: "foo".to_owned(),
                    timers: vec![x],
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        },
        Elements {
            timers: (0..=255)
                .map(|x| Timers {
                    instruction_id: "bar".to_owned(),
                    timers: vec![x],
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        },
        Elements {
            data: (0..128)
                .map(|x| Data {
                    instruction_id: "baz".to_owned(),
                    data: vec![x],
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        },
        Elements {
            data: (128..=255)
                .map(|x| Data {
                    instruction_id: "baz".to_owned(),
                    data: vec![x],
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        },
        Elements {
            timers: (0..128)
                .map(|x| Timers {
                    instruction_id: "qux".to_owned(),
                    timers: vec![x],
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        },
        Elements {
            timers: (128..=255)
                .map(|x| Timers {
                    instruction_id: "qux".to_owned(),
                    timers: vec![x],
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        },
    ];
}

// TODO(sjvanrossum): Perhaps use turmoil for these sorts of tests?
#[tokio::test]
async fn multiplex_to_consumers() {
    // Spawn a producer
    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(BeamFnDataServer::new(mock::DataProducer {}))
            .serve("[::1]:50051".parse().unwrap())
            .await
            .unwrap();
    });

    let mut client = apache_beam::worker::data::MultiplexingDataChannel::try_new(
        "hello_world".to_owned(),
        "http://[::1]:50051".to_owned(),
    )
    .unwrap();

    let (foo_tx, foo_rx) = mpsc::unbounded_channel();
    let (bar_tx, bar_rx) = mpsc::unbounded_channel();
    let (baz_tx, baz_rx) = mpsc::unbounded_channel();
    let (qux_tx, qux_rx) = mpsc::unbounded_channel();

    // Register consumers before starting
    client.register_consumer("foo".to_owned(), foo_tx);
    client.register_consumer("bar".to_owned(), bar_tx);
    client.register_consumer("baz".to_owned(), baz_tx);
    client.register_consumer("qux".to_owned(), qux_tx);

    tokio::spawn(async move {
        client.start().await.unwrap();
    });

    let foo_elements = UnboundedReceiverStream::from(foo_rx)
        .collect::<Vec<_>>()
        .await;
    let bar_elements = UnboundedReceiverStream::from(bar_rx)
        .collect::<Vec<_>>()
        .await;
    let baz_elements = UnboundedReceiverStream::from(baz_rx)
        .collect::<Vec<_>>()
        .await;
    let qux_elements = UnboundedReceiverStream::from(qux_rx)
        .collect::<Vec<_>>()
        .await;

    // Data and Timers in Elements at index 0
    assert_eq!(foo_elements.len(), 1);
    // Data in Elements at index 1 and Timers in Elements at index 2
    assert_eq!(bar_elements.len(), 2);
    // Data in Elements at index 3 and Data in Elements at index 4
    assert_eq!(baz_elements.len(), 2);
    // Timers in Elements at index 5 and Timers in Elements at index 6
    assert_eq!(qux_elements.len(), 2);

    // Ordering of Data and Timers within Elements must be preserved.
    let elements = (0..=255).collect::<Vec<_>>();
    assert_eq!(
        elements,
        foo_elements
            .iter()
            .flat_map(|e| &e.data)
            .flat_map(|d| &d.data)
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        elements,
        foo_elements
            .iter()
            .flat_map(|e| &e.timers)
            .flat_map(|t| &t.timers)
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        elements,
        bar_elements
            .iter()
            .flat_map(|e| &e.data)
            .flat_map(|d| &d.data)
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        elements,
        bar_elements
            .iter()
            .flat_map(|e| &e.timers)
            .flat_map(|t| &t.timers)
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        elements,
        baz_elements
            .iter()
            .flat_map(|e| &e.data)
            .flat_map(|d| &d.data)
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        elements,
        qux_elements
            .iter()
            .flat_map(|e| &e.timers)
            .flat_map(|t| &t.timers)
            .cloned()
            .collect::<Vec<_>>()
    );

    jh.abort();
}

// Essentially this test is somewhat pointless, since we rely on a mpsc channel.
#[tokio::test]
async fn multiplex_from_producers() {
    let received = AsyncCell::shared();

    // Spawn a consumer
    let stream = received.clone();
    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(BeamFnDataServer::new(mock::DataConsumer { stream }))
            .serve("[::1]:50052".parse().unwrap())
            .await
            .unwrap();
    });

    let mut client = apache_beam::worker::data::MultiplexingDataChannel::try_new(
        "hello_world".to_owned(),
        "http://[::1]:50052".to_owned(),
    )
    .unwrap();

    // Produce elements before starting
    let producer = client.get_producer();
    for es in ELEMENTS.iter() {
        producer.send(es.clone()).unwrap();
    }

    tokio::spawn(async move {
        client.start().await.unwrap();
    });

    let mut stream = received.take_shared().await;
    for es in ELEMENTS.iter() {
        assert_eq!(*es, stream.message().await.unwrap().unwrap());
    }

    jh.abort();
}
