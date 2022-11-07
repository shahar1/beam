/// A request for artifact resolution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveArtifactsRequest {
    /// An (ordered) set of artifacts to (jointly) resolve.
    #[prost(message, repeated, tag = "1")]
    pub artifacts: ::prost::alloc::vec::Vec<
        super::super::pipeline::v1::ArtifactInformation,
    >,
    /// A set of artifact type urns that are understood by the requester.
    /// An attempt should be made to resolve the artifacts in terms of these URNs,
    /// but other URNs may be used as well with the understanding that they must
    /// be fetch-able as bytes via GetArtifact.
    #[prost(string, repeated, tag = "2")]
    pub preferred_urns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A response for artifact resolution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveArtifactsResponse {
    /// A full (ordered) set of replacements for the set of requested artifacts,
    /// preferably in terms of the requested type URNs.  If there is no better
    /// resolution, the original list is returned.
    #[prost(message, repeated, tag = "1")]
    pub replacements: ::prost::alloc::vec::Vec<
        super::super::pipeline::v1::ArtifactInformation,
    >,
}
/// A request to get an artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArtifactRequest {
    #[prost(message, optional, tag = "1")]
    pub artifact: ::core::option::Option<
        super::super::pipeline::v1::ArtifactInformation,
    >,
}
/// Part of a response to getting an artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArtifactResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Wraps an ArtifactRetrievalService request for use in ReverseArtifactRetrievalService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactRequestWrapper {
    #[prost(oneof = "artifact_request_wrapper::Request", tags = "1000, 1001")]
    pub request: ::core::option::Option<artifact_request_wrapper::Request>,
}
/// Nested message and enum types in `ArtifactRequestWrapper`.
pub mod artifact_request_wrapper {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "1000")]
        ResolveArtifact(super::ResolveArtifactsRequest),
        #[prost(message, tag = "1001")]
        GetArtifact(super::GetArtifactRequest),
    }
}
/// Wraps an ArtifactRetrievalService response for use in ReverseArtifactRetrievalService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactResponseWrapper {
    /// A token indicating which job these artifacts are being staged for.
    #[prost(string, tag = "1")]
    pub staging_token: ::prost::alloc::string::String,
    /// Whether this is the last response for this request (for those responses that
    /// would typically be terminated by the end of the response stream.)
    #[prost(bool, tag = "2")]
    pub is_last: bool,
    /// The response itself.
    #[prost(oneof = "artifact_response_wrapper::Response", tags = "1000, 1001")]
    pub response: ::core::option::Option<artifact_response_wrapper::Response>,
}
/// Nested message and enum types in `ArtifactResponseWrapper`.
pub mod artifact_response_wrapper {
    /// The response itself.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1000")]
        ResolveArtifactResponse(super::ResolveArtifactsResponse),
        #[prost(message, tag = "1001")]
        GetArtifactResponse(super::GetArtifactResponse),
    }
}
/// An artifact identifier and associated metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactMetadata {
    /// (Required) The name of the artifact.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// (Optional) The Unix-like permissions of the artifact
    #[prost(uint32, tag = "2")]
    pub permissions: u32,
    /// (Optional) The hex-encoded sha256 checksum of the artifact. Used, among other things, by
    /// harness boot code to validate the integrity of the artifact.
    #[prost(string, tag = "4")]
    pub sha256: ::prost::alloc::string::String,
}
/// A collection of artifacts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manifest {
    #[prost(message, repeated, tag = "1")]
    pub artifact: ::prost::alloc::vec::Vec<ArtifactMetadata>,
}
/// A manifest with location information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyManifest {
    #[prost(message, optional, tag = "1")]
    pub manifest: ::core::option::Option<Manifest>,
    #[prost(message, repeated, tag = "2")]
    pub location: ::prost::alloc::vec::Vec<proxy_manifest::Location>,
}
/// Nested message and enum types in `ProxyManifest`.
pub mod proxy_manifest {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Location {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub uri: ::prost::alloc::string::String,
    }
}
/// A request to get the manifest of a Job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManifestRequest {
    /// (Required) An opaque token representing the entirety of the staged artifacts.
    /// Returned in CommitManifestResponse.
    #[prost(string, tag = "1")]
    pub retrieval_token: ::prost::alloc::string::String,
}
/// A response containing a job manifest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManifestResponse {
    #[prost(message, optional, tag = "1")]
    pub manifest: ::core::option::Option<Manifest>,
}
/// A request to get an artifact. The artifact must be present in the manifest for the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyGetArtifactRequest {
    /// (Required) The name of the artifact to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// (Required) An opaque token representing the entirety of the staged artifacts.
    /// Returned in CommitManifestResponse.
    #[prost(string, tag = "2")]
    pub retrieval_token: ::prost::alloc::string::String,
}
/// Part of an artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactChunk {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutArtifactMetadata {
    /// (Required) A token for artifact staging session. This token can be obtained
    /// from PrepareJob request in JobService
    #[prost(string, tag = "1")]
    pub staging_session_token: ::prost::alloc::string::String,
    /// (Required) The Artifact metadata.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ArtifactMetadata>,
}
/// A request to stage an artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutArtifactRequest {
    /// (Required)
    #[prost(oneof = "put_artifact_request::Content", tags = "1, 2")]
    pub content: ::core::option::Option<put_artifact_request::Content>,
}
/// Nested message and enum types in `PutArtifactRequest`.
pub mod put_artifact_request {
    /// (Required)
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// The first message in a PutArtifact call must contain this field.
        #[prost(message, tag = "1")]
        Metadata(super::PutArtifactMetadata),
        /// A chunk of the artifact. All messages after the first in a PutArtifact call must contain a
        /// chunk.
        #[prost(message, tag = "2")]
        Data(super::ArtifactChunk),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutArtifactResponse {}
/// A request to commit the manifest for a Job. All artifacts must have been successfully uploaded
/// before this call is made.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitManifestRequest {
    /// (Required) The manifest to commit.
    #[prost(message, optional, tag = "1")]
    pub manifest: ::core::option::Option<Manifest>,
    /// (Required) A token for artifact staging session. This token can be obtained
    /// from PrepareJob request in JobService
    #[prost(string, tag = "2")]
    pub staging_session_token: ::prost::alloc::string::String,
}
/// The result of committing a manifest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitManifestResponse {
    /// (Required) An opaque token representing the entirety of the staged artifacts.
    /// This can be used to retrieve the manifest and artifacts from an associated
    /// LegacyArtifactRetrievalService.
    #[prost(string, tag = "1")]
    pub retrieval_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CommitManifestResponse`.
pub mod commit_manifest_response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Constants {
        /// Token indicating that no artifacts were staged and therefore no retrieval attempt is necessary.
        NoArtifactsStagedToken = 0,
    }
    impl Constants {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Constants::NoArtifactsStagedToken => "NO_ARTIFACTS_STAGED_TOKEN",
            }
        }
    }
}
/// Generated client implementations.
pub mod artifact_retrieval_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service to retrieve artifacts for use in a Job.
    #[derive(Debug, Clone)]
    pub struct ArtifactRetrievalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ArtifactRetrievalServiceClient<tonic::transport::Channel> {
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
    impl<T> ArtifactRetrievalServiceClient<T>
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
        ) -> ArtifactRetrievalServiceClient<InterceptedService<T, F>>
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
            ArtifactRetrievalServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Resolves the given artifact references into one or more replacement
        /// artifact references (e.g. a Maven dependency into a (transitive) set
        /// of jars.
        pub async fn resolve_artifacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveArtifactsRequest>,
        ) -> Result<tonic::Response<super::ResolveArtifactsResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.ArtifactRetrievalService/ResolveArtifacts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the given artifact as a stream of bytes.
        pub async fn get_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::GetArtifactRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GetArtifactResponse>>,
            tonic::Status,
        > {
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
                "/org.apache.beam.model.job_management.v1.ArtifactRetrievalService/GetArtifact",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod artifact_staging_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service that allows the client to act as an ArtifactRetrievalService,
    /// for a particular job with the server initiating requests and receiving
    /// responses.
    ///
    /// A client calls the service with an ArtifactResponseWrapper that has the
    /// staging token set, and thereafter responds to the server's requests.
    #[derive(Debug, Clone)]
    pub struct ArtifactStagingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ArtifactStagingServiceClient<tonic::transport::Channel> {
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
    impl<T> ArtifactStagingServiceClient<T>
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
        ) -> ArtifactStagingServiceClient<InterceptedService<T, F>>
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
            ArtifactStagingServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn reverse_artifact_retrieval_service(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ArtifactResponseWrapper,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ArtifactRequestWrapper>>,
            tonic::Status,
        > {
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
                "/org.apache.beam.model.job_management.v1.ArtifactStagingService/ReverseArtifactRetrievalService",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod legacy_artifact_staging_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service to stage artifacts for use in a Job.
    #[derive(Debug, Clone)]
    pub struct LegacyArtifactStagingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LegacyArtifactStagingServiceClient<tonic::transport::Channel> {
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
    impl<T> LegacyArtifactStagingServiceClient<T>
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
        ) -> LegacyArtifactStagingServiceClient<InterceptedService<T, F>>
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
            LegacyArtifactStagingServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Stage an artifact to be available during job execution. The first request must contain the
        /// name of the artifact. All future requests must contain sequential chunks of the content of
        /// the artifact.
        pub async fn put_artifact(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::PutArtifactRequest,
            >,
        ) -> Result<tonic::Response<super::PutArtifactResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.LegacyArtifactStagingService/PutArtifact",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Commit the manifest for a Job. All artifacts must have been successfully uploaded
        /// before this call is made.
        ///
        /// Throws error INVALID_ARGUMENT if not all of the members of the manifest are present
        pub async fn commit_manifest(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitManifestRequest>,
        ) -> Result<tonic::Response<super::CommitManifestResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.LegacyArtifactStagingService/CommitManifest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod legacy_artifact_retrieval_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service to retrieve artifacts for use in a Job.
    #[derive(Debug, Clone)]
    pub struct LegacyArtifactRetrievalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LegacyArtifactRetrievalServiceClient<tonic::transport::Channel> {
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
    impl<T> LegacyArtifactRetrievalServiceClient<T>
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
        ) -> LegacyArtifactRetrievalServiceClient<InterceptedService<T, F>>
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
            LegacyArtifactRetrievalServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Get the manifest for the job
        pub async fn get_manifest(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManifestRequest>,
        ) -> Result<tonic::Response<super::GetManifestResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.LegacyArtifactRetrievalService/GetManifest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get an artifact staged for the job. The requested artifact must be within the manifest
        pub async fn get_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::LegacyGetArtifactRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ArtifactChunk>>,
            tonic::Status,
        > {
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
                "/org.apache.beam.model.job_management.v1.LegacyArtifactRetrievalService/GetArtifact",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod artifact_retrieval_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ArtifactRetrievalServiceServer.
    #[async_trait]
    pub trait ArtifactRetrievalService: Send + Sync + 'static {
        /// Resolves the given artifact references into one or more replacement
        /// artifact references (e.g. a Maven dependency into a (transitive) set
        /// of jars.
        async fn resolve_artifacts(
            &self,
            request: tonic::Request<super::ResolveArtifactsRequest>,
        ) -> Result<tonic::Response<super::ResolveArtifactsResponse>, tonic::Status>;
        ///Server streaming response type for the GetArtifact method.
        type GetArtifactStream: futures_core::Stream<
                Item = Result<super::GetArtifactResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Retrieves the given artifact as a stream of bytes.
        async fn get_artifact(
            &self,
            request: tonic::Request<super::GetArtifactRequest>,
        ) -> Result<tonic::Response<Self::GetArtifactStream>, tonic::Status>;
    }
    /// A service to retrieve artifacts for use in a Job.
    #[derive(Debug)]
    pub struct ArtifactRetrievalServiceServer<T: ArtifactRetrievalService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ArtifactRetrievalService> ArtifactRetrievalServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ArtifactRetrievalServiceServer<T>
    where
        T: ArtifactRetrievalService,
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
                "/org.apache.beam.model.job_management.v1.ArtifactRetrievalService/ResolveArtifacts" => {
                    #[allow(non_camel_case_types)]
                    struct ResolveArtifactsSvc<T: ArtifactRetrievalService>(pub Arc<T>);
                    impl<
                        T: ArtifactRetrievalService,
                    > tonic::server::UnaryService<super::ResolveArtifactsRequest>
                    for ResolveArtifactsSvc<T> {
                        type Response = super::ResolveArtifactsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResolveArtifactsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resolve_artifacts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResolveArtifactsSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.ArtifactRetrievalService/GetArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct GetArtifactSvc<T: ArtifactRetrievalService>(pub Arc<T>);
                    impl<
                        T: ArtifactRetrievalService,
                    > tonic::server::ServerStreamingService<super::GetArtifactRequest>
                    for GetArtifactSvc<T> {
                        type Response = super::GetArtifactResponse;
                        type ResponseStream = T::GetArtifactStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetArtifactRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_artifact(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetArtifactSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: ArtifactRetrievalService> Clone for ArtifactRetrievalServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ArtifactRetrievalService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ArtifactRetrievalService> tonic::server::NamedService
    for ArtifactRetrievalServiceServer<T> {
        const NAME: &'static str = "org.apache.beam.model.job_management.v1.ArtifactRetrievalService";
    }
}
/// Generated server implementations.
pub mod artifact_staging_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ArtifactStagingServiceServer.
    #[async_trait]
    pub trait ArtifactStagingService: Send + Sync + 'static {
        ///Server streaming response type for the ReverseArtifactRetrievalService method.
        type ReverseArtifactRetrievalServiceStream: futures_core::Stream<
                Item = Result<super::ArtifactRequestWrapper, tonic::Status>,
            >
            + Send
            + 'static;
        async fn reverse_artifact_retrieval_service(
            &self,
            request: tonic::Request<tonic::Streaming<super::ArtifactResponseWrapper>>,
        ) -> Result<
            tonic::Response<Self::ReverseArtifactRetrievalServiceStream>,
            tonic::Status,
        >;
    }
    /// A service that allows the client to act as an ArtifactRetrievalService,
    /// for a particular job with the server initiating requests and receiving
    /// responses.
    ///
    /// A client calls the service with an ArtifactResponseWrapper that has the
    /// staging token set, and thereafter responds to the server's requests.
    #[derive(Debug)]
    pub struct ArtifactStagingServiceServer<T: ArtifactStagingService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ArtifactStagingService> ArtifactStagingServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ArtifactStagingServiceServer<T>
    where
        T: ArtifactStagingService,
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
                "/org.apache.beam.model.job_management.v1.ArtifactStagingService/ReverseArtifactRetrievalService" => {
                    #[allow(non_camel_case_types)]
                    struct ReverseArtifactRetrievalServiceSvc<T: ArtifactStagingService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ArtifactStagingService,
                    > tonic::server::StreamingService<super::ArtifactResponseWrapper>
                    for ReverseArtifactRetrievalServiceSvc<T> {
                        type Response = super::ArtifactRequestWrapper;
                        type ResponseStream = T::ReverseArtifactRetrievalServiceStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ArtifactResponseWrapper>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).reverse_artifact_retrieval_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReverseArtifactRetrievalServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
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
    impl<T: ArtifactStagingService> Clone for ArtifactStagingServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ArtifactStagingService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ArtifactStagingService> tonic::server::NamedService
    for ArtifactStagingServiceServer<T> {
        const NAME: &'static str = "org.apache.beam.model.job_management.v1.ArtifactStagingService";
    }
}
/// Generated server implementations.
pub mod legacy_artifact_staging_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with LegacyArtifactStagingServiceServer.
    #[async_trait]
    pub trait LegacyArtifactStagingService: Send + Sync + 'static {
        /// Stage an artifact to be available during job execution. The first request must contain the
        /// name of the artifact. All future requests must contain sequential chunks of the content of
        /// the artifact.
        async fn put_artifact(
            &self,
            request: tonic::Request<tonic::Streaming<super::PutArtifactRequest>>,
        ) -> Result<tonic::Response<super::PutArtifactResponse>, tonic::Status>;
        /// Commit the manifest for a Job. All artifacts must have been successfully uploaded
        /// before this call is made.
        ///
        /// Throws error INVALID_ARGUMENT if not all of the members of the manifest are present
        async fn commit_manifest(
            &self,
            request: tonic::Request<super::CommitManifestRequest>,
        ) -> Result<tonic::Response<super::CommitManifestResponse>, tonic::Status>;
    }
    /// A service to stage artifacts for use in a Job.
    #[derive(Debug)]
    pub struct LegacyArtifactStagingServiceServer<T: LegacyArtifactStagingService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LegacyArtifactStagingService> LegacyArtifactStagingServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for LegacyArtifactStagingServiceServer<T>
    where
        T: LegacyArtifactStagingService,
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
                "/org.apache.beam.model.job_management.v1.LegacyArtifactStagingService/PutArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct PutArtifactSvc<T: LegacyArtifactStagingService>(pub Arc<T>);
                    impl<
                        T: LegacyArtifactStagingService,
                    > tonic::server::ClientStreamingService<super::PutArtifactRequest>
                    for PutArtifactSvc<T> {
                        type Response = super::PutArtifactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::PutArtifactRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).put_artifact(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutArtifactSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/org.apache.beam.model.job_management.v1.LegacyArtifactStagingService/CommitManifest" => {
                    #[allow(non_camel_case_types)]
                    struct CommitManifestSvc<T: LegacyArtifactStagingService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: LegacyArtifactStagingService,
                    > tonic::server::UnaryService<super::CommitManifestRequest>
                    for CommitManifestSvc<T> {
                        type Response = super::CommitManifestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommitManifestRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).commit_manifest(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CommitManifestSvc(inner);
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
    impl<T: LegacyArtifactStagingService> Clone
    for LegacyArtifactStagingServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: LegacyArtifactStagingService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LegacyArtifactStagingService> tonic::server::NamedService
    for LegacyArtifactStagingServiceServer<T> {
        const NAME: &'static str = "org.apache.beam.model.job_management.v1.LegacyArtifactStagingService";
    }
}
/// Generated server implementations.
pub mod legacy_artifact_retrieval_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with LegacyArtifactRetrievalServiceServer.
    #[async_trait]
    pub trait LegacyArtifactRetrievalService: Send + Sync + 'static {
        /// Get the manifest for the job
        async fn get_manifest(
            &self,
            request: tonic::Request<super::GetManifestRequest>,
        ) -> Result<tonic::Response<super::GetManifestResponse>, tonic::Status>;
        ///Server streaming response type for the GetArtifact method.
        type GetArtifactStream: futures_core::Stream<
                Item = Result<super::ArtifactChunk, tonic::Status>,
            >
            + Send
            + 'static;
        /// Get an artifact staged for the job. The requested artifact must be within the manifest
        async fn get_artifact(
            &self,
            request: tonic::Request<super::LegacyGetArtifactRequest>,
        ) -> Result<tonic::Response<Self::GetArtifactStream>, tonic::Status>;
    }
    /// A service to retrieve artifacts for use in a Job.
    #[derive(Debug)]
    pub struct LegacyArtifactRetrievalServiceServer<T: LegacyArtifactRetrievalService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LegacyArtifactRetrievalService> LegacyArtifactRetrievalServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for LegacyArtifactRetrievalServiceServer<T>
    where
        T: LegacyArtifactRetrievalService,
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
                "/org.apache.beam.model.job_management.v1.LegacyArtifactRetrievalService/GetManifest" => {
                    #[allow(non_camel_case_types)]
                    struct GetManifestSvc<T: LegacyArtifactRetrievalService>(pub Arc<T>);
                    impl<
                        T: LegacyArtifactRetrievalService,
                    > tonic::server::UnaryService<super::GetManifestRequest>
                    for GetManifestSvc<T> {
                        type Response = super::GetManifestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetManifestRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_manifest(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetManifestSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.LegacyArtifactRetrievalService/GetArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct GetArtifactSvc<T: LegacyArtifactRetrievalService>(pub Arc<T>);
                    impl<
                        T: LegacyArtifactRetrievalService,
                    > tonic::server::ServerStreamingService<
                        super::LegacyGetArtifactRequest,
                    > for GetArtifactSvc<T> {
                        type Response = super::ArtifactChunk;
                        type ResponseStream = T::GetArtifactStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LegacyGetArtifactRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_artifact(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetArtifactSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: LegacyArtifactRetrievalService> Clone
    for LegacyArtifactRetrievalServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: LegacyArtifactRetrievalService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LegacyArtifactRetrievalService> tonic::server::NamedService
    for LegacyArtifactRetrievalServiceServer<T> {
        const NAME: &'static str = "org.apache.beam.model.job_management.v1.LegacyArtifactRetrievalService";
    }
}
/// Prepare is a synchronous request that returns a preparationId back
/// Throws error GRPC_STATUS_UNAVAILABLE if server is down
/// Throws error ALREADY_EXISTS if the jobName is reused. Runners are permitted to deduplicate based on the name of the job.
/// Throws error UNKNOWN for all other issues
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareJobRequest {
    /// (required)
    #[prost(message, optional, tag = "1")]
    pub pipeline: ::core::option::Option<super::super::pipeline::v1::Pipeline>,
    /// (required)
    #[prost(message, optional, tag = "2")]
    pub pipeline_options: ::core::option::Option<::prost_types::Struct>,
    /// (required)
    #[prost(string, tag = "3")]
    pub job_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareJobResponse {
    /// (required) The ID used to associate calls made while preparing the job. preparationId is used
    /// to run the job.
    #[prost(string, tag = "1")]
    pub preparation_id: ::prost::alloc::string::String,
    /// An endpoint which exposes the Beam Artifact Staging API. Artifacts used by the job should be
    /// staged to this endpoint, and will be available during job execution.
    #[prost(message, optional, tag = "2")]
    pub artifact_staging_endpoint: ::core::option::Option<
        super::super::pipeline::v1::ApiServiceDescriptor,
    >,
    /// (required) Token for the artifact staging. This token also represent an artifact
    /// staging session with the artifact staging service.
    #[prost(string, tag = "3")]
    pub staging_session_token: ::prost::alloc::string::String,
}
/// Run is a synchronous request that returns a jobId back.
/// Throws error GRPC_STATUS_UNAVAILABLE if server is down
/// Throws error NOT_FOUND if the preparation ID does not exist
/// Throws error UNKNOWN for all other issues
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunJobRequest {
    /// (required) The ID provided by an earlier call to prepare. Runs the job. All prerequisite tasks
    /// must have been completed.
    #[prost(string, tag = "1")]
    pub preparation_id: ::prost::alloc::string::String,
    /// (optional) If any artifacts have been staged for this job, contains the retrieval_token returned
    /// from the CommitManifestResponse.
    #[prost(string, tag = "2")]
    pub retrieval_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunJobResponse {
    /// (required) The ID for the executing job
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
/// Cancel is a synchronus request that returns a job state back
/// Throws error GRPC_STATUS_UNAVAILABLE if server is down
/// Throws error NOT_FOUND if the jobId is not found
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelJobRequest {
    /// (required)
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
/// Valid responses include any terminal state or CANCELLING
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelJobResponse {
    /// (required)
    #[prost(enumeration = "job_state::Enum", tag = "1")]
    pub state: i32,
}
/// A subset of info provided by ProvisionApi.ProvisionInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobInfo {
    /// (required)
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    /// (required)
    #[prost(string, tag = "2")]
    pub job_name: ::prost::alloc::string::String,
    /// (required)
    #[prost(message, optional, tag = "3")]
    pub pipeline_options: ::core::option::Option<::prost_types::Struct>,
    /// (required)
    #[prost(enumeration = "job_state::Enum", tag = "4")]
    pub state: i32,
}
/// GetJobs is a synchronus request that returns a list of invoked jobs back
/// Throws error GRPC_STATUS_UNAVAILABLE if server is down
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobsResponse {
    /// (required)
    #[prost(message, repeated, tag = "1")]
    pub job_info: ::prost::alloc::vec::Vec<JobInfo>,
}
/// GetState is a synchronus request that returns a job state back
/// Throws error GRPC_STATUS_UNAVAILABLE if server is down
/// Throws error NOT_FOUND if the jobId is not found
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobStateRequest {
    /// (required)
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStateEvent {
    /// (required)
    #[prost(enumeration = "job_state::Enum", tag = "1")]
    pub state: i32,
    /// (required)
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
/// GetPipeline is a synchronus request that returns a pipeline back
/// Throws error GRPC_STATUS_UNAVAILABLE if server is down
/// Throws error NOT_FOUND if the jobId is not found
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobPipelineRequest {
    /// (required)
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobPipelineResponse {
    /// (required)
    #[prost(message, optional, tag = "1")]
    pub pipeline: ::core::option::Option<super::super::pipeline::v1::Pipeline>,
}
/// GetJobMessages is a streaming api for streaming job messages from the service
/// One request will connect you to the job and you'll get a stream of job state
/// and job messages back; one is used for logging and the other for detecting
/// the job ended.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobMessagesRequest {
    /// (required)
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobMessage {
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub time: ::prost::alloc::string::String,
    #[prost(enumeration = "job_message::MessageImportance", tag = "3")]
    pub importance: i32,
    #[prost(string, tag = "4")]
    pub message_text: ::prost::alloc::string::String,
}
/// Nested message and enum types in `JobMessage`.
pub mod job_message {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum MessageImportance {
        Unspecified = 0,
        JobMessageDebug = 1,
        JobMessageDetailed = 2,
        JobMessageBasic = 3,
        JobMessageWarning = 4,
        JobMessageError = 5,
    }
    impl MessageImportance {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MessageImportance::Unspecified => "MESSAGE_IMPORTANCE_UNSPECIFIED",
                MessageImportance::JobMessageDebug => "JOB_MESSAGE_DEBUG",
                MessageImportance::JobMessageDetailed => "JOB_MESSAGE_DETAILED",
                MessageImportance::JobMessageBasic => "JOB_MESSAGE_BASIC",
                MessageImportance::JobMessageWarning => "JOB_MESSAGE_WARNING",
                MessageImportance::JobMessageError => "JOB_MESSAGE_ERROR",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobMessagesResponse {
    #[prost(oneof = "job_messages_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<job_messages_response::Response>,
}
/// Nested message and enum types in `JobMessagesResponse`.
pub mod job_messages_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        MessageResponse(super::JobMessage),
        #[prost(message, tag = "2")]
        StateResponse(super::JobStateEvent),
    }
}
/// Enumeration of all JobStates
///
/// The state transition diagram is:
///    STOPPED -> STARTING -> RUNNING -> DONE
///                                   \> FAILED
///                                   \> CANCELLING -> CANCELLED
///                                   \> UPDATING -> UPDATED
///                                   \> DRAINING -> DRAINED
///
/// Transitions are optional such that a job may go from STOPPED to RUNNING
/// without needing to pass through STARTING.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobState {}
/// Nested message and enum types in `JobState`.
pub mod job_state {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Enum {
        /// The job state reported by a runner cannot be interpreted by the SDK.
        Unspecified = 0,
        /// The job has not yet started.
        Stopped = 1,
        /// The job is currently running.
        Running = 2,
        /// The job has successfully completed. (terminal)
        Done = 3,
        /// The job has failed. (terminal)
        Failed = 4,
        /// The job has been explicitly cancelled. (terminal)
        Cancelled = 5,
        /// The job has been updated. (terminal)
        Updated = 6,
        /// The job is draining its data. (optional)
        Draining = 7,
        /// The job has completed draining its data. (terminal)
        Drained = 8,
        /// The job is starting up.
        Starting = 9,
        /// The job is cancelling. (optional)
        Cancelling = 10,
        /// The job is in the process of being updated. (optional)
        Updating = 11,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Unspecified => "UNSPECIFIED",
                Enum::Stopped => "STOPPED",
                Enum::Running => "RUNNING",
                Enum::Done => "DONE",
                Enum::Failed => "FAILED",
                Enum::Cancelled => "CANCELLED",
                Enum::Updated => "UPDATED",
                Enum::Draining => "DRAINING",
                Enum::Drained => "DRAINED",
                Enum::Starting => "STARTING",
                Enum::Cancelling => "CANCELLING",
                Enum::Updating => "UPDATING",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobMetricsRequest {
    /// (required)
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobMetricsResponse {
    #[prost(message, optional, tag = "1")]
    pub metrics: ::core::option::Option<MetricResults>,
}
/// All metrics for a given job.  Runners may support one or the other or both.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricResults {
    #[prost(message, repeated, tag = "1")]
    pub attempted: ::prost::alloc::vec::Vec<super::super::pipeline::v1::MonitoringInfo>,
    #[prost(message, repeated, tag = "2")]
    pub committed: ::prost::alloc::vec::Vec<super::super::pipeline::v1::MonitoringInfo>,
}
/// DescribePipelineOptions provides metadata about the options supported by a runner.
/// It will be used by the SDK client to validate the options specified by or
/// list available options to the user.
/// Throws error GRPC_STATUS_UNAVAILABLE if server is down
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribePipelineOptionsRequest {}
/// Type for pipeline options.
/// Types mirror those of JSON, since that's how pipeline options are serialized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineOptionType {}
/// Nested message and enum types in `PipelineOptionType`.
pub mod pipeline_option_type {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Enum {
        String = 0,
        Boolean = 1,
        /// whole numbers, see <https://json-schema.org/understanding-json-schema/reference/numeric.html>
        Integer = 2,
        Number = 3,
        Array = 4,
        Object = 5,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::String => "STRING",
                Enum::Boolean => "BOOLEAN",
                Enum::Integer => "INTEGER",
                Enum::Number => "NUMBER",
                Enum::Array => "ARRAY",
                Enum::Object => "OBJECT",
            }
        }
    }
}
/// Metadata for a pipeline option.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineOptionDescriptor {
    /// (Required) The option name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// (Required) Type of option.
    #[prost(enumeration = "pipeline_option_type::Enum", tag = "2")]
    pub r#type: i32,
    /// (Optional) Description suitable for display / help text.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// (Optional) Default value.
    #[prost(string, tag = "4")]
    pub default_value: ::prost::alloc::string::String,
    /// (Required) The group this option belongs to.
    #[prost(string, tag = "5")]
    pub group: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribePipelineOptionsResponse {
    /// List of pipeline option descriptors.
    #[prost(message, repeated, tag = "1")]
    pub options: ::prost::alloc::vec::Vec<PipelineOptionDescriptor>,
}
/// Generated client implementations.
pub mod job_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Job Service for running RunnerAPI pipelines
    #[derive(Debug, Clone)]
    pub struct JobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl JobServiceClient<tonic::transport::Channel> {
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
    impl<T> JobServiceClient<T>
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
        ) -> JobServiceClient<InterceptedService<T, F>>
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
            JobServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Prepare a job for execution. The job will not be executed until a call is made to run with the
        /// returned preparationId.
        pub async fn prepare(
            &mut self,
            request: impl tonic::IntoRequest<super::PrepareJobRequest>,
        ) -> Result<tonic::Response<super::PrepareJobResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.JobService/Prepare",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Submit the job for execution
        pub async fn run(
            &mut self,
            request: impl tonic::IntoRequest<super::RunJobRequest>,
        ) -> Result<tonic::Response<super::RunJobResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.JobService/Run",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a list of all invoked jobs
        pub async fn get_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobsRequest>,
        ) -> Result<tonic::Response<super::GetJobsResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.JobService/GetJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get the current state of the job
        pub async fn get_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobStateRequest>,
        ) -> Result<tonic::Response<super::JobStateEvent>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.JobService/GetState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get the job's pipeline
        pub async fn get_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobPipelineRequest>,
        ) -> Result<tonic::Response<super::GetJobPipelineResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.JobService/GetPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancel the job
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelJobRequest>,
        ) -> Result<tonic::Response<super::CancelJobResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.JobService/Cancel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Subscribe to a stream of state changes of the job, will immediately return the current state of the job as the first response.
        pub async fn get_state_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::JobStateEvent>>,
            tonic::Status,
        > {
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
                "/org.apache.beam.model.job_management.v1.JobService/GetStateStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to a stream of state changes and messages from the job
        pub async fn get_message_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::JobMessagesRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::JobMessagesResponse>>,
            tonic::Status,
        > {
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
                "/org.apache.beam.model.job_management.v1.JobService/GetMessageStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Fetch metrics for a given job
        pub async fn get_job_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobMetricsRequest>,
        ) -> Result<tonic::Response<super::GetJobMetricsResponse>, tonic::Status> {
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
                "/org.apache.beam.model.job_management.v1.JobService/GetJobMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get the supported pipeline options of the runner
        pub async fn describe_pipeline_options(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribePipelineOptionsRequest>,
        ) -> Result<
            tonic::Response<super::DescribePipelineOptionsResponse>,
            tonic::Status,
        > {
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
                "/org.apache.beam.model.job_management.v1.JobService/DescribePipelineOptions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod job_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with JobServiceServer.
    #[async_trait]
    pub trait JobService: Send + Sync + 'static {
        /// Prepare a job for execution. The job will not be executed until a call is made to run with the
        /// returned preparationId.
        async fn prepare(
            &self,
            request: tonic::Request<super::PrepareJobRequest>,
        ) -> Result<tonic::Response<super::PrepareJobResponse>, tonic::Status>;
        /// Submit the job for execution
        async fn run(
            &self,
            request: tonic::Request<super::RunJobRequest>,
        ) -> Result<tonic::Response<super::RunJobResponse>, tonic::Status>;
        /// Get a list of all invoked jobs
        async fn get_jobs(
            &self,
            request: tonic::Request<super::GetJobsRequest>,
        ) -> Result<tonic::Response<super::GetJobsResponse>, tonic::Status>;
        /// Get the current state of the job
        async fn get_state(
            &self,
            request: tonic::Request<super::GetJobStateRequest>,
        ) -> Result<tonic::Response<super::JobStateEvent>, tonic::Status>;
        /// Get the job's pipeline
        async fn get_pipeline(
            &self,
            request: tonic::Request<super::GetJobPipelineRequest>,
        ) -> Result<tonic::Response<super::GetJobPipelineResponse>, tonic::Status>;
        /// Cancel the job
        async fn cancel(
            &self,
            request: tonic::Request<super::CancelJobRequest>,
        ) -> Result<tonic::Response<super::CancelJobResponse>, tonic::Status>;
        ///Server streaming response type for the GetStateStream method.
        type GetStateStreamStream: futures_core::Stream<
                Item = Result<super::JobStateEvent, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to a stream of state changes of the job, will immediately return the current state of the job as the first response.
        async fn get_state_stream(
            &self,
            request: tonic::Request<super::GetJobStateRequest>,
        ) -> Result<tonic::Response<Self::GetStateStreamStream>, tonic::Status>;
        ///Server streaming response type for the GetMessageStream method.
        type GetMessageStreamStream: futures_core::Stream<
                Item = Result<super::JobMessagesResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to a stream of state changes and messages from the job
        async fn get_message_stream(
            &self,
            request: tonic::Request<super::JobMessagesRequest>,
        ) -> Result<tonic::Response<Self::GetMessageStreamStream>, tonic::Status>;
        /// Fetch metrics for a given job
        async fn get_job_metrics(
            &self,
            request: tonic::Request<super::GetJobMetricsRequest>,
        ) -> Result<tonic::Response<super::GetJobMetricsResponse>, tonic::Status>;
        /// Get the supported pipeline options of the runner
        async fn describe_pipeline_options(
            &self,
            request: tonic::Request<super::DescribePipelineOptionsRequest>,
        ) -> Result<
            tonic::Response<super::DescribePipelineOptionsResponse>,
            tonic::Status,
        >;
    }
    /// Job Service for running RunnerAPI pipelines
    #[derive(Debug)]
    pub struct JobServiceServer<T: JobService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: JobService> JobServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for JobServiceServer<T>
    where
        T: JobService,
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
                "/org.apache.beam.model.job_management.v1.JobService/Prepare" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::UnaryService<super::PrepareJobRequest>
                    for PrepareSvc<T> {
                        type Response = super::PrepareJobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrepareJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).prepare(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.JobService/Run" => {
                    #[allow(non_camel_case_types)]
                    struct RunSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::RunJobRequest>
                    for RunSvc<T> {
                        type Response = super::RunJobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RunJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).run(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RunSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.JobService/GetJobs" => {
                    #[allow(non_camel_case_types)]
                    struct GetJobsSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::UnaryService<super::GetJobsRequest>
                    for GetJobsSvc<T> {
                        type Response = super::GetJobsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetJobsSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.JobService/GetState" => {
                    #[allow(non_camel_case_types)]
                    struct GetStateSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::UnaryService<super::GetJobStateRequest>
                    for GetStateSvc<T> {
                        type Response = super::JobStateEvent;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStateSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.JobService/GetPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct GetPipelineSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::UnaryService<super::GetJobPipelineRequest>
                    for GetPipelineSvc<T> {
                        type Response = super::GetJobPipelineResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobPipelineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_pipeline(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPipelineSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.JobService/Cancel" => {
                    #[allow(non_camel_case_types)]
                    struct CancelSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::UnaryService<super::CancelJobRequest>
                    for CancelSvc<T> {
                        type Response = super::CancelJobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.JobService/GetStateStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetStateStreamSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::ServerStreamingService<super::GetJobStateRequest>
                    for GetStateStreamSvc<T> {
                        type Response = super::JobStateEvent;
                        type ResponseStream = T::GetStateStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_state_stream(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStateStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/org.apache.beam.model.job_management.v1.JobService/GetMessageStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessageStreamSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::ServerStreamingService<super::JobMessagesRequest>
                    for GetMessageStreamSvc<T> {
                        type Response = super::JobMessagesResponse;
                        type ResponseStream = T::GetMessageStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::JobMessagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_message_stream(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMessageStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/org.apache.beam.model.job_management.v1.JobService/GetJobMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct GetJobMetricsSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::UnaryService<super::GetJobMetricsRequest>
                    for GetJobMetricsSvc<T> {
                        type Response = super::GetJobMetricsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobMetricsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_job_metrics(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetJobMetricsSvc(inner);
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
                "/org.apache.beam.model.job_management.v1.JobService/DescribePipelineOptions" => {
                    #[allow(non_camel_case_types)]
                    struct DescribePipelineOptionsSvc<T: JobService>(pub Arc<T>);
                    impl<
                        T: JobService,
                    > tonic::server::UnaryService<super::DescribePipelineOptionsRequest>
                    for DescribePipelineOptionsSvc<T> {
                        type Response = super::DescribePipelineOptionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DescribePipelineOptionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).describe_pipeline_options(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DescribePipelineOptionsSvc(inner);
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
    impl<T: JobService> Clone for JobServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: JobService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: JobService> tonic::server::NamedService for JobServiceServer<T> {
        const NAME: &'static str = "org.apache.beam.model.job_management.v1.JobService";
    }
}
