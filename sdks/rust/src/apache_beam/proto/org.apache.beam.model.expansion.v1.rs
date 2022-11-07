#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpansionRequest {
    /// Set of components needed to interpret the transform, or which
    /// may be useful for its expansion.  This includes the input
    /// PCollections (if any) to the to-be-expanded transform, along
    /// with their coders and windowing strategies.
    #[prost(message, optional, tag = "1")]
    pub components: ::core::option::Option<super::super::pipeline::v1::Components>,
    /// The actual PTransform to be expaneded according to its spec.
    /// Its input should be set, but its subtransforms and outputs
    /// should not be.
    #[prost(message, optional, tag = "2")]
    pub transform: ::core::option::Option<super::super::pipeline::v1::PTransform>,
    /// A namespace (prefix) to use for the id of any newly created
    /// components.
    #[prost(string, tag = "3")]
    pub namespace: ::prost::alloc::string::String,
    /// (Optional) Map from a local output tag to a coder id.
    /// If it is set, asks the expansion service to use the given
    /// coders for the output PCollections. Note that the request
    /// may not be fulfilled.
    #[prost(map = "string, string", tag = "4")]
    pub output_coder_requests: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpansionResponse {
    /// Set of components needed to execute the expanded transform,
    /// including the (original) inputs, outputs, and subtransforms.
    #[prost(message, optional, tag = "1")]
    pub components: ::core::option::Option<super::super::pipeline::v1::Components>,
    /// The expanded transform itself, with references to its outputs
    /// and subtransforms.
    #[prost(message, optional, tag = "2")]
    pub transform: ::core::option::Option<super::super::pipeline::v1::PTransform>,
    /// A set of requirements that must be appended to this pipeline's
    /// requirements.
    #[prost(string, repeated, tag = "3")]
    pub requirements: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// (Optional) An string representation of any error encountered while
    /// attempting to expand this transform.
    #[prost(string, tag = "10")]
    pub error: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod expansion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Job Service for constructing pipelines
    #[derive(Debug, Clone)]
    pub struct ExpansionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExpansionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ExpansionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ExpansionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ExpansionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn expand(
            &mut self,
            request: impl tonic::IntoRequest<super::ExpansionRequest>,
        ) -> Result<tonic::Response<super::ExpansionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.apache.beam.model.expansion.v1.ExpansionService/Expand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod expansion_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ExpansionServiceServer.
    #[async_trait]
    pub trait ExpansionService: Send + Sync + 'static {
        async fn expand(
            &self,
            request: tonic::Request<super::ExpansionRequest>,
        ) -> Result<tonic::Response<super::ExpansionResponse>, tonic::Status>;
    }
    /// Job Service for constructing pipelines
    #[derive(Debug)]
    pub struct ExpansionServiceServer<T: ExpansionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ExpansionService> ExpansionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExpansionServiceServer<T>
    where
        T: ExpansionService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/org.apache.beam.model.expansion.v1.ExpansionService/Expand" => {
                    #[allow(non_camel_case_types)]
                    struct ExpandSvc<T: ExpansionService>(pub Arc<T>);
                    impl<
                        T: ExpansionService,
                    > tonic::server::UnaryService<super::ExpansionRequest>
                    for ExpandSvc<T> {
                        type Response = super::ExpansionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExpansionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).expand(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExpandSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ExpansionService> Clone for ExpansionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ExpansionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ExpansionService> tonic::server::NamedService for ExpansionServiceServer<T> {
        const NAME: &'static str = "org.apache.beam.model.expansion.v1.ExpansionService";
    }
}
