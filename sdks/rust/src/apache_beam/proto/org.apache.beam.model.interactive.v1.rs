/// The first record. This contains metadata about the stream and how to
/// properly process it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestStreamFileHeader {
    /// The PCollection tag this stream is associated with.
    #[prost(string, tag = "1")]
    pub tag: ::prost::alloc::string::String,
}
/// A record is a recorded element that a source produced. Its function is to
/// give enough information to create a faithful recreation of the original
/// stream of data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestStreamFileRecord {
    /// The recorded event from an element stream.
    #[prost(message, optional, tag = "1")]
    pub recorded_event: ::core::option::Option<
        super::super::pipeline::v1::test_stream_payload::Event,
    >,
}
