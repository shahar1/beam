/// A description of how to connect to a Beam API endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiServiceDescriptor {
    /// (Required) The URL to connect to.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// (Optional) The method for authentication. If unspecified, access to the
    /// url is already being performed in a trusted context (e.g. localhost,
    /// private network).
    #[prost(message, optional, tag = "2")]
    pub authentication: ::core::option::Option<AuthenticationSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationSpec {
    /// (Required) A URN that describes the accompanying payload.
    /// For any URN that is not recognized (by whomever is inspecting
    /// it) the parameter payload should be treated as opaque and
    /// passed as-is.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// (Optional) The data specifying any parameters to the URN. If
    /// the URN does not require any arguments, this may be omitted.
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeamConstants {}
/// Nested message and enum types in `BeamConstants`.
pub mod beam_constants {
    /// All timestamps in milliseconds since Jan 1, 1970.
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
        /// All timestamps of elements or window boundaries must be within
        /// the interval [MIN_TIMESTAMP_MILLIS, MAX_TIMESTAMP_MILLIS].
        /// The smallest representable timestamp of an element or a window boundary.
        MinTimestampMillis = 0,
        /// The largest representable timestamp of an element or a window boundary.
        MaxTimestampMillis = 1,
        /// The maximum timestamp for the global window.
        /// Triggers use max timestamp to set timers' timestamp. Timers fire when
        /// the watermark passes their timestamps. So, the timestamp needs to be
        /// smaller than the MAX_TIMESTAMP_MILLIS.
        /// One standard day is subtracted from MAX_TIMESTAMP_MILLIS to make sure
        /// the max timestamp is smaller than MAX_TIMESTAMP_MILLIS even after rounding up
        /// to seconds or minutes.
        GlobalWindowMaxTimestampMillis = 2,
    }
    impl Constants {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Constants::MinTimestampMillis => "MIN_TIMESTAMP_MILLIS",
                Constants::MaxTimestampMillis => "MAX_TIMESTAMP_MILLIS",
                Constants::GlobalWindowMaxTimestampMillis => {
                    "GLOBAL_WINDOW_MAX_TIMESTAMP_MILLIS"
                }
            }
        }
    }
}
/// A set of mappings from id to message. This is included as an optional field
/// on any proto message that may contain references needing resolution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Components {
    /// (Required) A map from pipeline-scoped id to PTransform.
    ///
    /// Keys of the transforms map may be used by runners to identify pipeline
    /// steps. Hence it's recommended to use strings that are not too long that
    /// match regex '\[A-Za-z0-9-_\]+'.
    #[prost(map = "string, message", tag = "1")]
    pub transforms: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        PTransform,
    >,
    /// (Required) A map from pipeline-scoped id to PCollection.
    #[prost(map = "string, message", tag = "2")]
    pub pcollections: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        PCollection,
    >,
    /// (Required) A map from pipeline-scoped id to WindowingStrategy.
    #[prost(map = "string, message", tag = "3")]
    pub windowing_strategies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        WindowingStrategy,
    >,
    /// (Required) A map from pipeline-scoped id to Coder.
    #[prost(map = "string, message", tag = "4")]
    pub coders: ::std::collections::HashMap<::prost::alloc::string::String, Coder>,
    /// (Required) A map from pipeline-scoped id to Environment.
    #[prost(map = "string, message", tag = "5")]
    pub environments: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        Environment,
    >,
}
/// A Pipeline is a hierarchical graph of PTransforms, linked
/// by PCollections. A typical graph may look like:
///
///    Impulse -> PCollection -> ParDo -> PCollection -> GroupByKey -> ...
///                                    \> PCollection -> ParDo      -> ...
///                                                   \> ParDo      -> ...
///    Impulse -> PCollection -> ParDo -> PCollection -> ...
///
/// This is represented by a number of by-reference maps to transforms,
/// PCollections, SDK environments, coders, etc., for
/// supporting compact reuse and arbitrary graph structure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pipeline {
    /// (Required) The coders, UDFs, graph nodes, etc, that make up
    /// this pipeline.
    #[prost(message, optional, tag = "1")]
    pub components: ::core::option::Option<Components>,
    /// (Required) The ids of all PTransforms that are not contained within another
    /// PTransform. These must be in shallow topological order, so that traversing
    /// them recursively in this order yields a recursively topological traversal.
    #[prost(string, repeated, tag = "2")]
    pub root_transform_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// (Optional) Static display data for the pipeline. If there is none,
    /// it may be omitted.
    #[prost(message, repeated, tag = "3")]
    pub display_data: ::prost::alloc::vec::Vec<DisplayData>,
    /// (Optional) A set of requirements that the runner MUST understand and be
    /// able to faithfully provide in order to execute this pipeline. These
    /// may indicate that a runner must inspect new fields on a component or
    /// provide additional guarantees when processing specific transforms.
    /// A runner should reject any pipelines with unknown requirements.
    #[prost(string, repeated, tag = "4")]
    pub requirements: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Transforms are the operations in your pipeline, and provide a generic
/// processing framework. You provide processing logic in the form of a function
/// object (colloquially referred to as “user code”), and your user code is
/// applied to each element of an input PCollection (or more than one
/// PCollection). Depending on the pipeline runner and back-end that you choose,
/// many different workers across a cluster may execute instances of your user
/// code in parallel. The user code running on each worker generates the output
/// elements that are ultimately added to the final output PCollection that the
/// transform produces.
///
/// The Beam SDKs contain a number of different transforms that you can apply to
/// your pipeline’s PCollections. These include general-purpose core transforms,
/// such as ParDo or Combine. There are also pre-written composite transforms
/// included in the SDKs, which combine one or more of the core transforms in a
/// useful processing pattern, such as counting or combining elements in a
/// collection. You can also define your own more complex composite transforms to
/// fit your pipeline’s exact use case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PTransform {
    /// (Required) A unique name for the application node.
    ///
    /// Ideally, this should be stable over multiple evolutions of a pipeline
    /// for the purposes of logging and associating pipeline state with a node,
    /// etc.
    ///
    /// If it is not stable, then the runner decides what will happen. But, most
    /// importantly, it must always be here and be unique, even if it is
    /// autogenerated.
    #[prost(string, tag = "5")]
    pub unique_name: ::prost::alloc::string::String,
    /// (Optional) A URN and payload that, together, fully defined the semantics
    /// of this transform.
    ///
    /// If absent, this must be an "anonymous" composite transform.
    ///
    /// For primitive transform in the Runner API, this is required, and the
    /// payloads are well-defined messages. When the URN indicates ParDo it
    /// is a ParDoPayload, and so on. For some special composite transforms,
    /// the payload is also officially defined. See StandardPTransforms for
    /// details.
    #[prost(message, optional, tag = "1")]
    pub spec: ::core::option::Option<FunctionSpec>,
    /// (Optional) A list of the ids of transforms that it contains.
    ///
    /// Primitive transforms (see StandardPTransforms.Primitives) are not allowed
    /// to specify subtransforms.
    ///
    /// Note that a composite transform may have zero subtransforms as long as it
    /// only outputs PCollections that are in its inputs.
    #[prost(string, repeated, tag = "2")]
    pub subtransforms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// (Required) A map from local names of inputs (unique only with this map, and
    /// likely embedded in the transform payload and serialized user code) to
    /// PCollection ids.
    ///
    /// The payload for this transform may clarify the relationship of these
    /// inputs. For example:
    ///
    ///   - for a Flatten transform they are merged
    ///   - for a ParDo transform, some may be side inputs
    ///
    /// All inputs are recorded here so that the topological ordering of
    /// the graph is consistent whether or not the payload is understood.
    #[prost(map = "string, string", tag = "3")]
    pub inputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// (Required) A map from local names of outputs (unique only within this map,
    /// and likely embedded in the transform payload and serialized user code)
    /// to PCollection ids.
    ///
    /// The URN or payload for this transform node may clarify the type and
    /// relationship of these outputs. For example:
    ///
    ///   - for a ParDo transform, these are tags on PCollections, which will be
    ///     embedded in the DoFn.
    #[prost(map = "string, string", tag = "4")]
    pub outputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// (Optional) Static display data for this PTransform application. If
    /// there is none, it may be omitted.
    #[prost(message, repeated, tag = "6")]
    pub display_data: ::prost::alloc::vec::Vec<DisplayData>,
    /// Environment where the current PTransform should be executed in.
    ///
    /// Transforms that are required to be implemented by a runner must omit this.
    /// All other transforms are required to specify this.
    #[prost(string, tag = "7")]
    pub environment_id: ::prost::alloc::string::String,
    /// (Optional) A map from URNs designating a type of annotation, to the
    /// annotation in binary format. For example, an annotation could indicate
    /// that this PTransform has specific privacy properties.
    ///
    /// A runner MAY ignore types of annotations it doesn't understand. Therefore
    /// annotations MUST NOT be used for metadata that can affect correct
    /// execution of the transform.
    #[prost(map = "string, bytes", tag = "8")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardPTransforms {}
/// Nested message and enum types in `StandardPTransforms`.
pub mod standard_p_transforms {
    /// Primitive transforms may not specify composite sub-transforms.
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
    pub enum Primitives {
        /// ParDo is a Beam transform for generic parallel processing. The ParDo
        /// processing paradigm is similar to the “Map” phase of a
        /// Map/Shuffle/Reduce-style algorithm: a ParDo transform considers each
        /// element in the input PCollection, performs some processing function
        /// (your user code) on that element, and emits zero, one, or multiple
        /// elements to an output PCollection.
        ///
        /// See <https://beam.apache.org/documentation/programming-guide/#pardo>
        /// for additional details.
        ///
        /// Payload: ParDoPayload
        ParDo = 0,
        /// Flatten is a Beam transform for PCollection objects that store the same
        /// data type. Flatten merges multiple PCollection objects into a single
        /// logical PCollection.
        ///
        /// See <https://beam.apache.org/documentation/programming-guide/#flatten>
        /// for additional details.
        ///
        /// Payload: None
        Flatten = 1,
        /// GroupByKey is a Beam transform for processing collections of key/value
        /// pairs. It’s a parallel reduction operation, analogous to the Shuffle
        /// phase of a Map/Shuffle/Reduce-style algorithm. The input to GroupByKey is
        /// a collection of key/value pairs that represents a multimap, where the
        /// collection contains multiple pairs that have the same key, but different
        /// values. Given such a collection, you use GroupByKey to collect all of the
        /// values associated with each unique key.
        ///
        /// See <https://beam.apache.org/documentation/programming-guide/#groupbykey>
        /// for additional details.
        ///
        /// Never defines an environment as the runner is required to implement this
        /// transform.
        ///
        /// Payload: None
        GroupByKey = 2,
        /// A transform which produces a single empty byte array at the minimum
        /// timestamp in the GlobalWindow.
        ///
        /// Never defines an environment as the runner is required to implement this
        /// transform.
        ///
        /// Payload: None
        Impulse = 3,
        /// Windowing subdivides a PCollection according to the timestamps of its
        /// individual elements. Transforms that aggregate multiple elements, such as
        /// GroupByKey and Combine, work implicitly on a per-window basis — they
        /// process each PCollection as a succession of multiple, finite windows,
        /// though the entire collection itself may be of unbounded size.
        ///
        /// See <https://beam.apache.org/documentation/programming-guide/#windowing>
        /// for additional details.
        ///
        /// Payload: WindowIntoPayload
        AssignWindows = 4,
        /// A testing input that generates an unbounded {@link PCollection} of
        /// elements, advancing the watermark and processing time as elements are
        /// emitted. After all of the specified elements are emitted, ceases to
        /// produce output.
        ///
        /// See <https://beam.apache.org/blog/2016/10/20/test-stream.html>
        /// for additional details.
        ///
        /// Payload: TestStreamPayload
        TestStream = 5,
        /// Represents mapping of main input window onto side input window.
        ///
        /// Side input window mapping function:
        /// Input: KV<nonce, MainInputWindow>
        /// Output: KV<nonce, SideInputWindow>
        ///
        /// For each main input window, the side input window is returned. The
        /// nonce is used by a runner to associate each input with its output.
        /// The nonce is represented as an opaque set of bytes.
        ///
        /// Payload: SideInput#window_mapping_fn FunctionSpec
        MapWindows = 6,
        /// Used to merge windows during a GroupByKey.
        ///
        /// Window merging function:
        /// Input: KV<nonce, iterable<OriginalWindow>>
        /// Output: KV<nonce, KV<iterable<UnmergedOriginalWindow>, iterable<KV<MergedWindow, iterable<ConsumedOriginalWindow>>>>
        ///
        /// For each set of original windows, a list of all unmerged windows is
        /// output alongside a map of merged window to set of consumed windows.
        /// All original windows must be contained in either the unmerged original
        /// window set or one of the consumed original window sets. Each original
        /// window can only be part of one output set. The nonce is used by a runner
        /// to associate each input with its output. The nonce is represented as an
        /// opaque set of bytes.
        ///
        /// Payload: WindowingStrategy#window_fn FunctionSpec
        MergeWindows = 7,
        /// A transform that translates a given element to its human-readable
        /// representation.
        ///
        /// Input: KV<nonce, element>
        /// Output: KV<nonce, string>
        ///
        /// For each given element, the implementation returns the best-effort
        /// human-readable representation. When possible, the implementation could
        /// call a user-overridable method on the type. For example, Java could
        /// call `toString()`, Python could call `str()`, Golang could call
        /// `String()`.  The nonce is used by a runner to associate each input with
        /// its output. The nonce is represented as an opaque set of bytes.
        ///
        /// Payload: none
        ToString = 8,
    }
    impl Primitives {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Primitives::ParDo => "PAR_DO",
                Primitives::Flatten => "FLATTEN",
                Primitives::GroupByKey => "GROUP_BY_KEY",
                Primitives::Impulse => "IMPULSE",
                Primitives::AssignWindows => "ASSIGN_WINDOWS",
                Primitives::TestStream => "TEST_STREAM",
                Primitives::MapWindows => "MAP_WINDOWS",
                Primitives::MergeWindows => "MERGE_WINDOWS",
                Primitives::ToString => "TO_STRING",
            }
        }
    }
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
    pub enum DeprecatedPrimitives {
        /// Represents the operation to read a Bounded or Unbounded source.
        /// Payload: ReadPayload.
        Read = 0,
        /// Runners should move away from translating `CreatePCollectionView` and treat this as
        /// part of the translation for a `ParDo` side input.
        CreateView = 1,
    }
    impl DeprecatedPrimitives {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DeprecatedPrimitives::Read => "READ",
                DeprecatedPrimitives::CreateView => "CREATE_VIEW",
            }
        }
    }
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
    pub enum Composites {
        /// Represents the Combine.perKey() operation.
        /// If this is produced by an SDK, it is assumed that the SDK understands
        /// each of CombineComponents.
        /// Payload: CombinePayload
        CombinePerKey = 0,
        /// Represents the Combine.globally() operation.
        /// If this is produced by an SDK, it is assumed that the SDK understands
        /// each of CombineComponents.
        /// Payload: CombinePayload
        CombineGlobally = 1,
        /// Represents the Reshuffle operation.
        Reshuffle = 2,
        /// Less well-known. Payload: WriteFilesPayload.
        WriteFiles = 3,
        /// Payload: PubSubReadPayload.
        PubsubRead = 4,
        /// Payload: PubSubWritePayload.
        PubsubWrite = 5,
        /// Used for pubsub dynamic destinations.
        /// Payload: PubSubWritePayload.
        PubsubWriteV2 = 7,
        /// Represents the GroupIntoBatches.WithShardedKey operation.
        /// Payload: GroupIntoBatchesPayload
        GroupIntoBatchesWithShardedKey = 6,
    }
    impl Composites {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Composites::CombinePerKey => "COMBINE_PER_KEY",
                Composites::CombineGlobally => "COMBINE_GLOBALLY",
                Composites::Reshuffle => "RESHUFFLE",
                Composites::WriteFiles => "WRITE_FILES",
                Composites::PubsubRead => "PUBSUB_READ",
                Composites::PubsubWrite => "PUBSUB_WRITE",
                Composites::PubsubWriteV2 => "PUBSUB_WRITE_V2",
                Composites::GroupIntoBatchesWithShardedKey => {
                    "GROUP_INTO_BATCHES_WITH_SHARDED_KEY"
                }
            }
        }
    }
    /// Payload for all of these: CombinePayload
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
    pub enum CombineComponents {
        /// Represents the Pre-Combine part of a lifted Combine Per Key, as described
        /// in the following document:
        /// <https://s.apache.org/beam-runner-api-combine-model#heading=h.ta0g6ase8z07>
        /// Payload: CombinePayload
        CombinePerKeyPrecombine = 0,
        /// Represents the Merge Accumulators part of a lifted Combine Per Key, as
        /// described in the following document:
        /// <https://s.apache.org/beam-runner-api-combine-model#heading=h.jco9rvatld5m>
        /// Payload: CombinePayload
        CombinePerKeyMergeAccumulators = 1,
        /// Represents the Extract Outputs part of a lifted Combine Per Key, as
        /// described in the following document:
        /// <https://s.apache.org/beam-runner-api-combine-model#heading=h.i9i6p8gtl6ku>
        /// Payload: CombinePayload
        CombinePerKeyExtractOutputs = 2,
        /// Represents the Combine Grouped Values transform, as described in the
        /// following document:
        /// <https://s.apache.org/beam-runner-api-combine-model#heading=h.aj86ew4v1wk>
        /// Payload: CombinePayload
        CombineGroupedValues = 3,
        /// Represents the Convert To Accumulators transform, as described in the
        /// following document:
        /// <https://s.apache.org/beam-runner-api-combine-model#heading=h.h5697l1scd9x>
        /// Payload: CombinePayload
        CombinePerKeyConvertToAccumulators = 4,
    }
    impl CombineComponents {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CombineComponents::CombinePerKeyPrecombine => {
                    "COMBINE_PER_KEY_PRECOMBINE"
                }
                CombineComponents::CombinePerKeyMergeAccumulators => {
                    "COMBINE_PER_KEY_MERGE_ACCUMULATORS"
                }
                CombineComponents::CombinePerKeyExtractOutputs => {
                    "COMBINE_PER_KEY_EXTRACT_OUTPUTS"
                }
                CombineComponents::CombineGroupedValues => "COMBINE_GROUPED_VALUES",
                CombineComponents::CombinePerKeyConvertToAccumulators => {
                    "COMBINE_PER_KEY_CONVERT_TO_ACCUMULATORS"
                }
            }
        }
    }
    /// Payload for all of these: ParDoPayload containing the user's SDF
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
    pub enum SplittableParDoComponents {
        /// Pairs the input element with its initial restriction.
        /// Input: element; output: KV(element, restriction).
        PairWithRestriction = 0,
        /// Splits the restriction of each element/restriction pair and returns the
        /// resulting splits, with a corresponding floating point size estimation
        /// for each.
        ///
        /// A reasonable value for size is the number of bytes expected to be
        /// produced by this (element, restriction) pair.
        ///
        /// Input: KV(element, restriction)
        /// Output: KV(KV(element, restriction), size))
        SplitAndSizeRestrictions = 1,
        /// Applies the DoFn to every element and restriction.
        ///
        /// All primary and residuals returned from checkpointing or splitting must
        /// have the same type as the input to this transform.
        ///
        /// Input: KV(KV(element, restriction), size); output: DoFn's output.
        ProcessSizedElementsAndRestrictions = 2,
        /// Truncates the restriction of each element/restriction pair and returns
        /// the finite restriction which will be processed when a pipeline is
        /// drained. See
        /// <https://docs.google.com/document/d/1NExwHlj-2q2WUGhSO4jTu8XGhDPmm3cllSN8IMmWci8/edit#.>
        /// for additional details about drain.
        ///
        /// Input: KV(KV(element, restriction), size);
        /// Output: KV(KV(element, restriction), size).
        TruncateSizedRestriction = 3,
    }
    impl SplittableParDoComponents {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SplittableParDoComponents::PairWithRestriction => "PAIR_WITH_RESTRICTION",
                SplittableParDoComponents::SplitAndSizeRestrictions => {
                    "SPLIT_AND_SIZE_RESTRICTIONS"
                }
                SplittableParDoComponents::ProcessSizedElementsAndRestrictions => {
                    "PROCESS_SIZED_ELEMENTS_AND_RESTRICTIONS"
                }
                SplittableParDoComponents::TruncateSizedRestriction => {
                    "TRUNCATE_SIZED_RESTRICTION"
                }
            }
        }
    }
    /// Payload for all of these: GroupIntoBatchesPayload
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
    pub enum GroupIntoBatchesComponents {
        GroupIntoBatches = 0,
    }
    impl GroupIntoBatchesComponents {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GroupIntoBatchesComponents::GroupIntoBatches => "GROUP_INTO_BATCHES",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardSideInputTypes {}
/// Nested message and enum types in `StandardSideInputTypes`.
pub mod standard_side_input_types {
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
        /// Represents a view over a PCollection<V>.
        ///
        /// StateGetRequests performed on this side input must use
        /// StateKey.IterableSideInput.
        Iterable = 0,
        /// Represents a view over a PCollection<KV<K, V>>.
        ///
        /// StateGetRequests performed on this side input must use
        /// StateKey.MultimapKeysSideInput or StateKey.MultimapSideInput.
        Multimap = 1,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Iterable => "ITERABLE",
                Enum::Multimap => "MULTIMAP",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardUserStateTypes {}
/// Nested message and enum types in `StandardUserStateTypes`.
pub mod standard_user_state_types {
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
        /// Represents a user state specification that supports a bag.
        ///
        /// StateRequests performed on this user state must use
        /// StateKey.BagUserState.
        Bag = 0,
        /// Represents a user state specification that supports a multimap.
        ///
        /// StateRequests performed on this user state must use
        /// StateKey.MultimapKeysUserState or StateKey.MultimapUserState.
        Multimap = 1,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Bag => "BAG",
                Enum::Multimap => "MULTIMAP",
            }
        }
    }
}
/// A PCollection!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PCollection {
    /// (Required) A unique name for the PCollection.
    ///
    /// Ideally, this should be stable over multiple evolutions of a pipeline
    /// for the purposes of logging and associating pipeline state with a node,
    /// etc.
    ///
    /// If it is not stable, then the runner decides what will happen. But, most
    /// importantly, it must always be here, even if it is autogenerated.
    #[prost(string, tag = "1")]
    pub unique_name: ::prost::alloc::string::String,
    /// (Required) The id of the Coder for this PCollection.
    #[prost(string, tag = "2")]
    pub coder_id: ::prost::alloc::string::String,
    /// (Required) Whether this PCollection is bounded or unbounded
    #[prost(enumeration = "is_bounded::Enum", tag = "3")]
    pub is_bounded: i32,
    /// (Required) The id of the windowing strategy for this PCollection.
    #[prost(string, tag = "4")]
    pub windowing_strategy_id: ::prost::alloc::string::String,
    /// (Optional) Static display data for the PCollection. If there is none,
    /// it may be omitted.
    #[prost(message, repeated, tag = "5")]
    pub display_data: ::prost::alloc::vec::Vec<DisplayData>,
}
/// The payload for the primitive ParDo transform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParDoPayload {
    /// (Required) The FunctionSpec of the DoFn.
    #[prost(message, optional, tag = "1")]
    pub do_fn: ::core::option::Option<FunctionSpec>,
    /// (Optional) A mapping of local input names to side inputs, describing
    /// the expected access pattern.
    #[prost(map = "string, message", tag = "3")]
    pub side_inputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        SideInput,
    >,
    /// (Optional) A mapping of local state names to state specifications.
    /// If this is set, the stateful processing requirement should also
    /// be placed in the pipeline requirements.
    #[prost(map = "string, message", tag = "4")]
    pub state_specs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        StateSpec,
    >,
    /// (Optional) A mapping of local timer family names to timer family
    /// specifications. If this is set, the stateful processing requirement should
    /// also be placed in the pipeline requirements.
    #[prost(map = "string, message", tag = "9")]
    pub timer_family_specs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        TimerFamilySpec,
    >,
    /// (Optional) Only set when this ParDo contains a splittable DoFn.
    /// If this is set, the corresponding standard requirement should also
    /// be placed in the pipeline requirements.
    #[prost(string, tag = "7")]
    pub restriction_coder_id: ::prost::alloc::string::String,
    /// (Optional) Only set when this ParDo can request bundle finalization.
    /// If this is set, the corresponding standard requirement should also
    /// be placed in the pipeline requirements.
    #[prost(bool, tag = "8")]
    pub requests_finalization: bool,
    /// Whether this stage requires time sorted input.
    /// If this is set, the corresponding standard requirement should also
    /// be placed in the pipeline requirements.
    #[prost(bool, tag = "10")]
    pub requires_time_sorted_input: bool,
    /// Whether this stage requires stable input.
    /// If this is set, the corresponding standard requirement should also
    /// be placed in the pipeline requirements.
    #[prost(bool, tag = "11")]
    pub requires_stable_input: bool,
    /// If populated, the name of the timer family spec which should be notified
    /// on each window expiry.
    /// If this is set, the corresponding standard requirement should also
    /// be placed in the pipeline requirements.
    #[prost(string, tag = "12")]
    pub on_window_expiration_timer_family_spec: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateSpec {
    /// (Required) URN of the protocol required by this state specification to present
    /// the desired SDK-specific interface to a UDF.
    ///
    /// This protocol defines the SDK harness <-> Runner Harness RPC
    /// interface for accessing and mutating user state.
    ///
    /// See StandardUserStateTypes for an enumeration of all user state types
    /// defined.
    #[prost(message, optional, tag = "7")]
    pub protocol: ::core::option::Option<FunctionSpec>,
    /// TODO(BEAM-13930): Deprecate and remove these state specs
    #[prost(oneof = "state_spec::Spec", tags = "1, 2, 3, 4, 5, 6")]
    pub spec: ::core::option::Option<state_spec::Spec>,
}
/// Nested message and enum types in `StateSpec`.
pub mod state_spec {
    /// TODO(BEAM-13930): Deprecate and remove these state specs
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Spec {
        #[prost(message, tag = "1")]
        ReadModifyWriteSpec(super::ReadModifyWriteStateSpec),
        #[prost(message, tag = "2")]
        BagSpec(super::BagStateSpec),
        #[prost(message, tag = "3")]
        CombiningSpec(super::CombiningStateSpec),
        #[prost(message, tag = "4")]
        MapSpec(super::MapStateSpec),
        #[prost(message, tag = "5")]
        SetSpec(super::SetStateSpec),
        #[prost(message, tag = "6")]
        OrderedListSpec(super::OrderedListStateSpec),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadModifyWriteStateSpec {
    #[prost(string, tag = "1")]
    pub coder_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BagStateSpec {
    #[prost(string, tag = "1")]
    pub element_coder_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderedListStateSpec {
    #[prost(string, tag = "1")]
    pub element_coder_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombiningStateSpec {
    #[prost(string, tag = "1")]
    pub accumulator_coder_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub combine_fn: ::core::option::Option<FunctionSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapStateSpec {
    #[prost(string, tag = "1")]
    pub key_coder_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value_coder_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetStateSpec {
    #[prost(string, tag = "1")]
    pub element_coder_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimerFamilySpec {
    #[prost(enumeration = "time_domain::Enum", tag = "1")]
    pub time_domain: i32,
    #[prost(string, tag = "2")]
    pub timer_family_coder_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsBounded {}
/// Nested message and enum types in `IsBounded`.
pub mod is_bounded {
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
        Unspecified = 0,
        Unbounded = 1,
        Bounded = 2,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Unspecified => "UNSPECIFIED",
                Enum::Unbounded => "UNBOUNDED",
                Enum::Bounded => "BOUNDED",
            }
        }
    }
}
/// The payload for the primitive Read transform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadPayload {
    /// (Required) The FunctionSpec of the source for this Read.
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<FunctionSpec>,
    /// (Required) Whether the source is bounded or unbounded
    #[prost(enumeration = "is_bounded::Enum", tag = "2")]
    pub is_bounded: i32,
}
/// The payload for the WindowInto transform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowIntoPayload {
    /// (Required) The FunctionSpec of the WindowFn.
    #[prost(message, optional, tag = "1")]
    pub window_fn: ::core::option::Option<FunctionSpec>,
}
/// The payload for the special-but-not-primitive Combine transform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinePayload {
    /// (Required) The FunctionSpec of the CombineFn.
    #[prost(message, optional, tag = "1")]
    pub combine_fn: ::core::option::Option<FunctionSpec>,
    /// (Required) A reference to the Coder to use for accumulators of the CombineFn
    #[prost(string, tag = "2")]
    pub accumulator_coder_id: ::prost::alloc::string::String,
}
/// The payload for the test-only primitive TestStream
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestStreamPayload {
    /// (Required) the coder for elements in the TestStream events
    #[prost(string, tag = "1")]
    pub coder_id: ::prost::alloc::string::String,
    /// (Optional) If specified, the TestStream will replay these events.
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<test_stream_payload::Event>,
    /// (Optional) If specified, points to a TestStreamService to be
    /// used to retrieve events.
    #[prost(message, optional, tag = "3")]
    pub endpoint: ::core::option::Option<ApiServiceDescriptor>,
}
/// Nested message and enum types in `TestStreamPayload`.
pub mod test_stream_payload {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Event {
        #[prost(oneof = "event::Event", tags = "1, 2, 3")]
        pub event: ::core::option::Option<event::Event>,
    }
    /// Nested message and enum types in `Event`.
    pub mod event {
        /// Advances the watermark to the specified timestamp.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AdvanceWatermark {
            /// (Required) The watermark in millisecond to advance to.
            #[prost(int64, tag = "1")]
            pub new_watermark: i64,
            /// (Optional) The output watermark tag for a PCollection. If unspecified
            /// or with an empty string, this will default to the Main PCollection
            /// Output
            #[prost(string, tag = "2")]
            pub tag: ::prost::alloc::string::String,
        }
        /// Advances the processing time clock by the specified amount.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AdvanceProcessingTime {
            /// (Required) The duration in millisecond to advance by.
            #[prost(int64, tag = "1")]
            pub advance_duration: i64,
        }
        /// Adds elements to the stream to be emitted.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AddElements {
            /// (Required) The elements to add to the TestStream.
            #[prost(message, repeated, tag = "1")]
            pub elements: ::prost::alloc::vec::Vec<super::TimestampedElement>,
            /// (Optional) The output PCollection tag to add these elements to. If
            /// unspecified or with an empty string, this will default to the Main
            /// PCollection Output.
            #[prost(string, tag = "3")]
            pub tag: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Event {
            #[prost(message, tag = "1")]
            WatermarkEvent(AdvanceWatermark),
            #[prost(message, tag = "2")]
            ProcessingTimeEvent(AdvanceProcessingTime),
            #[prost(message, tag = "3")]
            ElementEvent(AddElements),
        }
    }
    /// A single element inside of the TestStream.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimestampedElement {
        /// (Required) The element encoded. Currently the TestStream only supports
        /// encoding primitives.
        #[prost(bytes = "vec", tag = "1")]
        pub encoded_element: ::prost::alloc::vec::Vec<u8>,
        /// (Required) The event timestamp in millisecond of this element.
        #[prost(int64, tag = "2")]
        pub timestamp: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventsRequest {
    /// The set of PCollections to read from. These are the PTransform outputs
    /// local names. These are a subset of the TestStream's outputs. This allows
    /// Interactive Beam to cache many PCollections from a pipeline then replay a
    /// subset of them.
    #[prost(string, repeated, tag = "1")]
    pub output_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The payload for the special-but-not-primitive WriteFiles transform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteFilesPayload {
    /// (Required) The FunctionSpec of the FileBasedSink.
    #[prost(message, optional, tag = "1")]
    pub sink: ::core::option::Option<FunctionSpec>,
    /// (Required) The format function.
    #[prost(message, optional, tag = "2")]
    pub format_function: ::core::option::Option<FunctionSpec>,
    #[prost(bool, tag = "3")]
    pub windowed_writes: bool,
    #[prost(bool, tag = "4")]
    pub runner_determined_sharding: bool,
    #[prost(map = "string, message", tag = "5")]
    pub side_inputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        SideInput,
    >,
}
/// Payload used by Google Cloud Pub/Sub read transform.
/// This can be used by runners that wish to override Beam Pub/Sub read transform
/// with a native implementation.
/// The SDK should guarantee that only one of topic, subscription,
/// topic_runtime_overridden and subscription_runtime_overridden is set.
/// The output of PubSubReadPayload should be bytes of serialized PubsubMessage
/// proto if with_attributes == true. Otherwise, the bytes is the raw payload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubSubReadPayload {
    /// Topic to read from. Exactly one of topic or subscription should be set.
    /// Topic format is: /topics/project_id/subscription_name
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// Subscription to read from. Exactly one of topic or subscription should be set.
    /// Subscription format is: /subscriptions/project_id/subscription_name
    #[prost(string, tag = "2")]
    pub subscription: ::prost::alloc::string::String,
    /// Attribute that provides element timestamps.
    #[prost(string, tag = "3")]
    pub timestamp_attribute: ::prost::alloc::string::String,
    /// Attribute to be used for uniquely identifying messages.
    #[prost(string, tag = "4")]
    pub id_attribute: ::prost::alloc::string::String,
    /// If true, reads Pub/Sub payload as well as attributes. If false, reads only the payload.
    #[prost(bool, tag = "5")]
    pub with_attributes: bool,
    /// If set, the topic is expected to be provided during runtime.
    #[prost(string, tag = "6")]
    pub topic_runtime_overridden: ::prost::alloc::string::String,
    /// If set, the subscription that is expected to be provided during runtime.
    #[prost(string, tag = "7")]
    pub subscription_runtime_overridden: ::prost::alloc::string::String,
}
/// Payload used by Google Cloud Pub/Sub write transform.
/// This can be used by runners that wish to override Beam Pub/Sub write transform
/// with a native implementation.
/// The SDK should guarantee that only one of topic and topic_runtime_overridden
/// is set.
/// The output of PubSubWritePayload should be bytes if serialized PubsubMessage
/// proto.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubSubWritePayload {
    /// Topic to write to.
    /// Topic format is: /topics/project_id/subscription_name
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// Attribute that provides element timestamps.
    #[prost(string, tag = "2")]
    pub timestamp_attribute: ::prost::alloc::string::String,
    /// Attribute that uniquely identify messages.
    #[prost(string, tag = "3")]
    pub id_attribute: ::prost::alloc::string::String,
    /// If set, the topic is expected to be provided during runtime.
    #[prost(string, tag = "4")]
    pub topic_runtime_overridden: ::prost::alloc::string::String,
}
/// Payload for GroupIntoBatches composite transform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupIntoBatchesPayload {
    /// Max size of a batch.
    #[prost(int64, tag = "1")]
    pub batch_size: i64,
    /// Max byte size of a batch in element.
    #[prost(int64, tag = "3")]
    pub batch_size_bytes: i64,
    /// (Optional) Max duration a batch is allowed to be cached in states.
    #[prost(int64, tag = "2")]
    pub max_buffering_duration_millis: i64,
}
/// A coder, the binary format for serialization and deserialization of data in
/// a pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coder {
    /// (Required) A specification for the coder, as a URN plus parameters. This
    /// may be a cross-language agreed-upon format, or it may be a "custom coder"
    /// that can only be used by a particular SDK. It does not include component
    /// coders, as it is beneficial for these to be comprehensible to a runner
    /// regardless of whether the binary format is agreed-upon.
    #[prost(message, optional, tag = "1")]
    pub spec: ::core::option::Option<FunctionSpec>,
    /// (Optional) If this coder is parametric, such as ListCoder(VarIntCoder),
    /// this is a list of the components. In order for encodings to be identical,
    /// the FunctionSpec and all components must be identical, recursively.
    #[prost(string, repeated, tag = "2")]
    pub component_coder_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardCoders {}
/// Nested message and enum types in `StandardCoders`.
pub mod standard_coders {
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
        /// Components: None
        Bytes = 0,
        /// Components: None
        StringUtf8 = 10,
        /// Components: The key and value coder, in that order.
        Kv = 1,
        /// Components: None
        Bool = 12,
        /// Variable length Encodes a 64-bit integer.
        /// Components: None
        Varint = 2,
        /// Encodes the floating point value as a big-endian 64-bit integer
        /// according to the IEEE 754 double format bit layout.
        /// Components: None
        Double = 11,
        /// Encodes an iterable of elements.
        ///
        /// The encoding for an iterable \[e1...eN\] of known length N is
        ///
        ///     fixed32(N)
        ///     encode(e1) encode(e2) encode(e3) ... encode(eN)
        ///
        /// If the length is unknown, it is batched up into groups of size b1..bM
        /// and encoded as
        ///
        ///      fixed32(-1)
        ///      varInt64(b1) encode(e1) encode(e2) ... encode(e_b1)
        ///      varInt64(b2) encode(e_(b1+1)) encode(e_(b1+2)) ... encode(e_(b1+b2))
        ///      ...
        ///      varInt64(bM) encode(e_(N-bM+1)) encode(e_(N-bM+2)) ... encode(eN)
        ///      varInt64(0)
        ///
        /// Components: Coder for a single element.
        Iterable = 3,
        /// Encodes a timer containing a user key, a dynamic timer tag, a clear bit,
        /// a fire timestamp, a hold timestamp, the windows and the paneinfo.
        /// The encoding is represented as:
        ///    user key - user defined key, uses the component coder.
        ///    dynamic timer tag - a string which identifies a timer.
        ///    windows - uses component coders.
        ///    clear bit - a boolean set for clearing the timer.
        ///    fire timestamp - a big endian 8 byte integer representing millis-since-epoch.
        ///      The encoded representation is shifted so that the byte representation of
        ///      negative values are lexicographically ordered before the byte representation
        ///      of positive values. This is typically done by subtracting -9223372036854775808
        ///      from the value and encoding it as a signed big endian integer. Example values:
        ///
        ///      -9223372036854775808: 00 00 00 00 00 00 00 00
        ///                      -255: 7F FF FF FF FF FF FF 01
        ///                        -1: 7F FF FF FF FF FF FF FF
        ///                         0: 80 00 00 00 00 00 00 00
        ///                         1: 80 00 00 00 00 00 00 01
        ///                       256: 80 00 00 00 00 00 01 00
        ///       9223372036854775807: FF FF FF FF FF FF FF FF
        ///    hold timestamp - similar to the fire timestamp.
        ///    paneinfo - similar to the paneinfo of the windowed_value.
        /// Components: Coder for the key and windows.
        Timer = 4,
        /// Components: None
        IntervalWindow = 5,
        /// Components: The coder to attach a length prefix to
        LengthPrefix = 6,
        /// Components: None
        GlobalWindow = 7,
        /// Encodes an element, the windows it is in, the timestamp of the element,
        /// and the pane of the element. The encoding is represented as:
        /// timestamp windows pane element
        ///    timestamp - A big endian 8 byte integer representing millis-since-epoch.
        ///      The encoded representation is shifted so that the byte representation
        ///      of negative values are lexicographically ordered before the byte
        ///      representation of positive values. This is typically done by
        ///      subtracting -9223372036854775808 from the value and encoding it as a
        ///      signed big endian integer. Example values:
        ///
        ///      -9223372036854775808: 00 00 00 00 00 00 00 00
        ///                      -255: 7F FF FF FF FF FF FF 01
        ///                        -1: 7F FF FF FF FF FF FF FF
        ///                         0: 80 00 00 00 00 00 00 00
        ///                         1: 80 00 00 00 00 00 00 01
        ///                       256: 80 00 00 00 00 00 01 00
        ///       9223372036854775807: FF FF FF FF FF FF FF FF
        ///
        ///    windows - The windows are encoded using the beam:coder:iterable:v1
        ///      format, where the windows are encoded using the supplied window
        ///      coder.
        ///
        ///    pane - The first byte of the pane info determines which type of
        ///      encoding is used, as well as the is_first, is_last, and timing
        ///      fields. If this byte is bits [0 1 2 3 4 5 6 7], then:
        ///      * bits [0 1 2 3] determine the encoding as follows:
        ///          0000 - The entire pane info is encoded as a single byte.
        ///                 The is_first, is_last, and timing fields are encoded
        ///                 as below, and the index and non-speculative index are
        ///                 both zero (and hence are not encoded here).
        ///          0001 - The pane info is encoded as this byte plus a single
        ///                 VarInt encoed integer representing the pane index. The
        ///                 non-speculative index can be derived as follows:
        ///                   -1 if the pane is early, otherwise equal to index.
        ///          0010 - The pane info is encoded as this byte plus two VarInt
        ///                 encoded integers representing the pane index and
        ///                 non-speculative index respectively.
        ///      * bits [4 5] encode the timing as follows:
        ///          00 - early
        ///          01 - on time
        ///          10 - late
        ///          11 - unknown
        ///      * bit 6 is 1 if this is the first pane, 0 otherwise.
        ///      * bit 7 is 1 if this is the last pane, 0 otherwise.
        ///
        ///    element - The element incoded using the supplied element coder.
        ///
        /// Components: The element coder and the window coder, in that order.
        WindowedValue = 8,
        /// A windowed value coder with parameterized timestamp, windows and pane info.
        /// Encodes an element with only the value of the windowed value.
        /// Decodes the value and assigns the parameterized timestamp, windows and pane info to the
        /// windowed value.
        /// Components: The element coder and the window coder, in that order
        /// The payload of this coder is an encoded windowed value using the
        /// beam:coder:windowed_value:v1 coder parameterized by a beam:coder:bytes:v1
        /// element coder and the window coder that this param_windowed_value coder uses.
        ParamWindowedValue = 14,
        /// Encodes an iterable of elements, some of which may be stored elsewhere.
        ///
        /// The encoding for a state-backed iterable is the same as that for
        /// an iterable, but the final varInt64(0) terminating the set of batches
        /// may instead be replaced by
        ///
        ///      varInt64(-1)
        ///      varInt64(len(token))
        ///      token
        ///
        /// where token is an opaque byte string that can be used to fetch the
        /// remainder of the iterable (e.g. over the state API).
        ///
        /// Components: Coder for a single element.
        /// Experimental.
        StateBackedIterable = 9,
        /// Encodes an arbitrary user defined window and its max timestamp (inclusive).
        /// The encoding format is:
        ///    maxTimestamp window
        ///
        ///    maxTimestamp - A big endian 8 byte integer representing millis-since-epoch.
        ///      The encoded representation is shifted so that the byte representation
        ///      of negative values are lexicographically ordered before the byte
        ///      representation of positive values. This is typically done by
        ///      subtracting -9223372036854775808 from the value and encoding it as a
        ///      signed big endian integer. Example values:
        ///
        ///      -9223372036854775808: 00 00 00 00 00 00 00 00
        ///                      -255: 7F FF FF FF FF FF FF 01
        ///                        -1: 7F FF FF FF FF FF FF FF
        ///                         0: 80 00 00 00 00 00 00 00
        ///                         1: 80 00 00 00 00 00 00 01
        ///                       256: 80 00 00 00 00 00 01 00
        ///       9223372036854775807: FF FF FF FF FF FF FF FF
        ///
        ///    window - the window is encoded using the supplied window coder.
        ///
        /// Components: Coder for the custom window type.
        CustomWindow = 16,
        /// Encodes a "row", an element with a known schema, defined by an
        /// instance of Schema from schema.proto.
        ///
        /// A row is encoded as the concatenation of:
        ///    - The number of attributes in the schema, encoded with
        ///      beam:coder:varint:v1. This makes it possible to detect certain
        ///      allowed schema changes (appending or removing columns) in
        ///      long-running streaming pipelines.
        ///    - A byte array representing a packed bitset indicating null fields (a
        ///      1 indicating a null) encoded with beam:coder:bytes:v1. The unused
        ///      bits in the last byte must be set to 0. If there are no nulls an
        ///      empty byte array is encoded.
        ///      The two-byte bitset (not including the lenghth-prefix) for the row
        ///      [NULL, 0, 0, 0, NULL, 0, 0, NULL, 0, NULL] would be
        ///      [0b10010001, 0b00000010]
        ///    - An encoding for each non-null field, concatenated together.
        ///
        /// Schema types are mapped to coders as follows:
        ///    AtomicType:
        ///      BYTE:      not yet a standard coder (<https://github.com/apache/beam/issues/19815>)
        ///      INT16:     not yet a standard coder (<https://github.com/apache/beam/issues/19815>)
        ///      INT32:     beam:coder:varint:v1
        ///      INT64:     beam:coder:varint:v1
        ///      FLOAT:     not yet a standard coder (<https://github.com/apache/beam/issues/19815>)
        ///      DOUBLE:    beam:coder:double:v1
        ///      STRING:    beam:coder:string_utf8:v1
        ///      BOOLEAN:   beam:coder:bool:v1
        ///      BYTES:     beam:coder:bytes:v1
        ///    ArrayType:   beam:coder:iterable:v1 (always has a known length)
        ///    MapType:     not a standard coder, specification defined below.
        ///    RowType:     beam:coder:row:v1
        ///    LogicalType: Uses the coder for its representation.
        ///
        /// The MapType is encoded by:
        ///    - An INT32 representing the size of the map (N)
        ///    - Followed by N interleaved keys and values, encoded with their
        ///      corresponding coder.
        ///
        /// Nullable types in container types (ArrayType, MapType) per the
        /// encoding described for general Nullable types below.
        ///
        /// Logical types understood by all SDKs should be defined in schema.proto.
        /// Example of well known logical types:
        ///    beam:logical_type:schema:v1
        ///    - Representation type: BYTES
        ///    - A Beam Schema stored as a serialized proto.
        ///
        /// The payload for RowCoder is an instance of Schema.
        /// Components: None
        /// Experimental.
        Row = 13,
        /// Encodes a user key and a shard id which is an opaque byte string.
        ///
        /// The encoding for a sharded key consists of a shard id byte string and the
        /// encoded user key in the following order:
        ///
        ///      - shard id using beam:coder:bytes:v1
        ///      - encoded user key
        ///
        /// Examples:
        /// user key with an empty shard id
        ///      0x00
        ///      encode(user_key)
        ///
        /// user key with a shard id taking up two bytes.
        ///      0x02
        ///      0x11 0x22
        ///      encode(user_key)
        ///
        /// Components: the user key coder.
        /// Experimental.
        ShardedKey = 15,
        /// Wraps a coder of a potentially null value
        /// A Nullable Type is encoded by:
        ///    - A one byte null indicator, 0x00 for null values, or 0x01 for present
        ///      values.
        ///    - For present values the null indicator is followed by the value
        ///      encoded with it's corresponding coder.
        /// Components: single coder for the value
        Nullable = 17,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Bytes => "BYTES",
                Enum::StringUtf8 => "STRING_UTF8",
                Enum::Kv => "KV",
                Enum::Bool => "BOOL",
                Enum::Varint => "VARINT",
                Enum::Double => "DOUBLE",
                Enum::Iterable => "ITERABLE",
                Enum::Timer => "TIMER",
                Enum::IntervalWindow => "INTERVAL_WINDOW",
                Enum::LengthPrefix => "LENGTH_PREFIX",
                Enum::GlobalWindow => "GLOBAL_WINDOW",
                Enum::WindowedValue => "WINDOWED_VALUE",
                Enum::ParamWindowedValue => "PARAM_WINDOWED_VALUE",
                Enum::StateBackedIterable => "STATE_BACKED_ITERABLE",
                Enum::CustomWindow => "CUSTOM_WINDOW",
                Enum::Row => "ROW",
                Enum::ShardedKey => "SHARDED_KEY",
                Enum::Nullable => "NULLABLE",
            }
        }
    }
}
/// A windowing strategy describes the window function, triggering, allowed
/// lateness, and accumulation mode for a PCollection.
///
/// TODO: consider inlining field on PCollection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowingStrategy {
    /// (Required) The FunctionSpec of the UDF that assigns windows,
    /// merges windows, and shifts timestamps before they are
    /// combined according to the OutputTime.
    #[prost(message, optional, tag = "1")]
    pub window_fn: ::core::option::Option<FunctionSpec>,
    /// (Required) Whether or not the window fn is merging.
    ///
    /// This knowledge is required for many optimizations.
    #[prost(enumeration = "merge_status::Enum", tag = "2")]
    pub merge_status: i32,
    /// (Required) The coder for the windows of this PCollection.
    #[prost(string, tag = "3")]
    pub window_coder_id: ::prost::alloc::string::String,
    /// (Required) The trigger to use when grouping this PCollection.
    #[prost(message, optional, tag = "4")]
    pub trigger: ::core::option::Option<Trigger>,
    /// (Required) The accumulation mode indicates whether new panes are a full
    /// replacement for prior panes or whether they are deltas to be combined
    /// with other panes (the combine should correspond to whatever the upstream
    /// grouping transform is).
    #[prost(enumeration = "accumulation_mode::Enum", tag = "5")]
    pub accumulation_mode: i32,
    /// (Required) The OutputTime specifies, for a grouping transform, how to
    /// compute the aggregate timestamp. The window_fn will first possibly shift
    /// it later, then the OutputTime takes the max, min, or ignores it and takes
    /// the end of window.
    ///
    /// This is actually only for input to grouping transforms, but since they
    /// may be introduced in runner-specific ways, it is carried along with the
    /// windowing strategy.
    #[prost(enumeration = "output_time::Enum", tag = "6")]
    pub output_time: i32,
    /// (Required) Indicate when output should be omitted upon window expiration.
    #[prost(enumeration = "closing_behavior::Enum", tag = "7")]
    pub closing_behavior: i32,
    /// (Required) The duration, in milliseconds, beyond the end of a window at
    /// which the window becomes droppable.
    #[prost(int64, tag = "8")]
    pub allowed_lateness: i64,
    /// (Required) Indicate whether empty on-time panes should be omitted.
    #[prost(enumeration = "on_time_behavior::Enum", tag = "9")]
    pub on_time_behavior: i32,
    /// (Required) Whether or not the window fn assigns inputs to exactly one window
    ///
    /// This knowledge is required for some optimizations
    #[prost(bool, tag = "10")]
    pub assigns_to_one_window: bool,
    /// (Optional) Environment where the current window_fn should be applied in.
    /// Runner that executes the pipeline may choose to override this if needed.
    /// If not specified, environment will be decided by the runner.
    #[prost(string, tag = "11")]
    pub environment_id: ::prost::alloc::string::String,
}
/// Whether or not a PCollection's WindowFn is non-merging, merging, or
/// merging-but-already-merged, in which case a subsequent GroupByKey is almost
/// always going to do something the user does not want
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeStatus {}
/// Nested message and enum types in `MergeStatus`.
pub mod merge_status {
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
        Unspecified = 0,
        /// The WindowFn does not require merging.
        /// Examples: global window, FixedWindows, SlidingWindows
        NonMerging = 1,
        /// The WindowFn is merging and the PCollection has not had merging
        /// performed.
        /// Example: Sessions prior to a GroupByKey
        NeedsMerge = 2,
        /// The WindowFn is merging and the PCollection has had merging occur
        /// already.
        /// Example: Sessions after a GroupByKey
        AlreadyMerged = 3,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Unspecified => "UNSPECIFIED",
                Enum::NonMerging => "NON_MERGING",
                Enum::NeedsMerge => "NEEDS_MERGE",
                Enum::AlreadyMerged => "ALREADY_MERGED",
            }
        }
    }
}
/// Whether or not subsequent outputs of aggregations should be entire
/// replacement values or just the aggregation of inputs received since
/// the prior output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccumulationMode {}
/// Nested message and enum types in `AccumulationMode`.
pub mod accumulation_mode {
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
        Unspecified = 0,
        /// The aggregation is discarded when it is output
        Discarding = 1,
        /// The aggregation is accumulated across outputs
        Accumulating = 2,
        /// The aggregation emits retractions when it is output
        Retracting = 3,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Unspecified => "UNSPECIFIED",
                Enum::Discarding => "DISCARDING",
                Enum::Accumulating => "ACCUMULATING",
                Enum::Retracting => "RETRACTING",
            }
        }
    }
}
/// Controls whether or not an aggregating transform should output data
/// when a window expires.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosingBehavior {}
/// Nested message and enum types in `ClosingBehavior`.
pub mod closing_behavior {
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
        Unspecified = 0,
        /// Emit output when a window expires, whether or not there has been
        /// any new data since the last output.
        EmitAlways = 1,
        /// Only emit output when new data has arrives since the last output
        EmitIfNonempty = 2,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Unspecified => "UNSPECIFIED",
                Enum::EmitAlways => "EMIT_ALWAYS",
                Enum::EmitIfNonempty => "EMIT_IF_NONEMPTY",
            }
        }
    }
}
/// Controls whether or not an aggregating transform should output data
/// when an on-time pane is empty.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnTimeBehavior {}
/// Nested message and enum types in `OnTimeBehavior`.
pub mod on_time_behavior {
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
        Unspecified = 0,
        /// Always fire the on-time pane. Even if there is no new data since
        /// the previous firing, an element will be produced.
        FireAlways = 1,
        /// Only fire the on-time pane if there is new data since the previous firing.
        FireIfNonempty = 2,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Unspecified => "UNSPECIFIED",
                Enum::FireAlways => "FIRE_ALWAYS",
                Enum::FireIfNonempty => "FIRE_IF_NONEMPTY",
            }
        }
    }
}
/// When a number of windowed, timestamped inputs are aggregated, the timestamp
/// for the resulting output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputTime {}
/// Nested message and enum types in `OutputTime`.
pub mod output_time {
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
        Unspecified = 0,
        /// The output has the timestamp of the end of the window.
        EndOfWindow = 1,
        /// The output has the latest timestamp of the input elements since
        /// the last output.
        LatestInPane = 2,
        /// The output has the earliest timestamp of the input elements since
        /// the last output.
        EarliestInPane = 3,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Unspecified => "UNSPECIFIED",
                Enum::EndOfWindow => "END_OF_WINDOW",
                Enum::LatestInPane => "LATEST_IN_PANE",
                Enum::EarliestInPane => "EARLIEST_IN_PANE",
            }
        }
    }
}
/// The different time domains in the Beam model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeDomain {}
/// Nested message and enum types in `TimeDomain`.
pub mod time_domain {
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
        Unspecified = 0,
        /// Event time is time from the perspective of the data
        EventTime = 1,
        /// Processing time is time from the perspective of the
        /// execution of your pipeline
        ProcessingTime = 2,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Unspecified => "UNSPECIFIED",
                Enum::EventTime => "EVENT_TIME",
                Enum::ProcessingTime => "PROCESSING_TIME",
            }
        }
    }
}
/// A small DSL for expressing when to emit new aggregations
/// from a GroupByKey or CombinePerKey
///
/// A trigger is described in terms of when it is _ready_ to permit output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    /// The full disjoint union of possible triggers.
    #[prost(oneof = "trigger::Trigger", tags = "1, 2, 3, 4, 5, 6, 12, 7, 8, 9, 10, 11")]
    pub trigger: ::core::option::Option<trigger::Trigger>,
}
/// Nested message and enum types in `Trigger`.
pub mod trigger {
    /// Ready when all subtriggers are ready.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AfterAll {
        #[prost(message, repeated, tag = "1")]
        pub subtriggers: ::prost::alloc::vec::Vec<super::Trigger>,
    }
    /// Ready when any subtrigger is ready.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AfterAny {
        #[prost(message, repeated, tag = "1")]
        pub subtriggers: ::prost::alloc::vec::Vec<super::Trigger>,
    }
    /// Starting with the first subtrigger, ready when the _current_ subtrigger
    /// is ready. After output, advances the current trigger by one.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AfterEach {
        #[prost(message, repeated, tag = "1")]
        pub subtriggers: ::prost::alloc::vec::Vec<super::Trigger>,
    }
    /// Ready after the input watermark is past the end of the window.
    ///
    /// May have implicitly-repeated subtriggers for early and late firings.
    /// When the end of the window is reached, the trigger transitions between
    /// the subtriggers.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AfterEndOfWindow {
        /// (Optional) A trigger governing output prior to the end of the window.
        #[prost(message, optional, boxed, tag = "1")]
        pub early_firings: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Trigger>,
        >,
        /// (Optional) A trigger governing output after the end of the window.
        #[prost(message, optional, boxed, tag = "2")]
        pub late_firings: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Trigger>,
        >,
    }
    /// After input arrives, ready when the specified delay has passed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AfterProcessingTime {
        /// (Required) The transforms to apply to an arriving element's timestamp,
        /// in order
        #[prost(message, repeated, tag = "1")]
        pub timestamp_transforms: ::prost::alloc::vec::Vec<super::TimestampTransform>,
    }
    /// Ready whenever upstream processing time has all caught up with
    /// the arrival time of an input element
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AfterSynchronizedProcessingTime {}
    /// The default trigger. Equivalent to Repeat { AfterEndOfWindow } but
    /// specially denoted to indicate the user did not alter the triggering.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Default {}
    /// Ready whenever the requisite number of input elements have arrived
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ElementCount {
        #[prost(int32, tag = "1")]
        pub element_count: i32,
    }
    /// Never ready. There will only be an ON_TIME output and a final
    /// output at window expiration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Never {}
    /// Always ready. This can also be expressed as ElementCount(1) but
    /// is more explicit.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Always {}
    /// Ready whenever either of its subtriggers are ready, but finishes output
    /// when the finally subtrigger fires.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrFinally {
        /// (Required) Trigger governing main output; may fire repeatedly.
        #[prost(message, optional, boxed, tag = "1")]
        pub main: ::core::option::Option<::prost::alloc::boxed::Box<super::Trigger>>,
        /// (Required) Trigger governing termination of output.
        #[prost(message, optional, boxed, tag = "2")]
        pub finally: ::core::option::Option<::prost::alloc::boxed::Box<super::Trigger>>,
    }
    /// Ready whenever the subtrigger is ready; resets state when the subtrigger
    /// completes.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Repeat {
        /// (Require) Trigger that is run repeatedly.
        #[prost(message, optional, boxed, tag = "1")]
        pub subtrigger: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Trigger>,
        >,
    }
    /// The full disjoint union of possible triggers.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        #[prost(message, tag = "1")]
        AfterAll(AfterAll),
        #[prost(message, tag = "2")]
        AfterAny(AfterAny),
        #[prost(message, tag = "3")]
        AfterEach(AfterEach),
        #[prost(message, tag = "4")]
        AfterEndOfWindow(::prost::alloc::boxed::Box<AfterEndOfWindow>),
        #[prost(message, tag = "5")]
        AfterProcessingTime(AfterProcessingTime),
        #[prost(message, tag = "6")]
        AfterSynchronizedProcessingTime(AfterSynchronizedProcessingTime),
        #[prost(message, tag = "12")]
        Always(Always),
        #[prost(message, tag = "7")]
        Default(Default),
        #[prost(message, tag = "8")]
        ElementCount(ElementCount),
        #[prost(message, tag = "9")]
        Never(Never),
        #[prost(message, tag = "10")]
        OrFinally(::prost::alloc::boxed::Box<OrFinally>),
        #[prost(message, tag = "11")]
        Repeat(::prost::alloc::boxed::Box<Repeat>),
    }
}
/// A specification for a transformation on a timestamp.
///
/// Primarily used by AfterProcessingTime triggers to transform
/// the arrival time of input to a target time for firing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampTransform {
    #[prost(oneof = "timestamp_transform::TimestampTransform", tags = "1, 2")]
    pub timestamp_transform: ::core::option::Option<
        timestamp_transform::TimestampTransform,
    >,
}
/// Nested message and enum types in `TimestampTransform`.
pub mod timestamp_transform {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delay {
        /// (Required) The delay, in milliseconds.
        #[prost(int64, tag = "1")]
        pub delay_millis: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AlignTo {
        /// (Required) A duration to which delays should be quantized
        /// in milliseconds.
        #[prost(int64, tag = "3")]
        pub period: i64,
        /// (Required) An offset from 0 for the quantization specified by
        /// alignment_size, in milliseconds
        #[prost(int64, tag = "4")]
        pub offset: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TimestampTransform {
        #[prost(message, tag = "1")]
        Delay(Delay),
        #[prost(message, tag = "2")]
        AlignTo(AlignTo),
    }
}
/// A specification for how to "side input" a PCollection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SideInput {
    /// (Required) URN of the access pattern required by the `view_fn` to present
    /// the desired SDK-specific interface to a UDF.
    ///
    /// This access pattern defines the SDK harness <-> Runner Harness RPC
    /// interface for accessing a side input.
    ///
    /// See StandardSideInputTypes for an enumeration of all side input types
    /// defined.
    #[prost(message, optional, tag = "1")]
    pub access_pattern: ::core::option::Option<FunctionSpec>,
    /// (Required) The FunctionSpec of the UDF that adapts a particular
    /// access_pattern to a user-facing view type.
    ///
    /// For example, View.asSingleton() may include a `view_fn` that adapts a
    /// specially-designed multimap to a single value per window.
    #[prost(message, optional, tag = "2")]
    pub view_fn: ::core::option::Option<FunctionSpec>,
    /// (Required) The FunctionSpec of the UDF that maps a main input window
    /// to a side input window.
    ///
    /// For example, when the main input is in fixed windows of one hour, this
    /// can specify that the side input should be accessed according to the day
    /// in which that hour falls.
    #[prost(message, optional, tag = "3")]
    pub window_mapping_fn: ::core::option::Option<FunctionSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardArtifacts {}
/// Nested message and enum types in `StandardArtifacts`.
pub mod standard_artifacts {
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
    pub enum Types {
        /// A URN for locally-accessible artifact files.
        /// payload: ArtifactFilePayload
        File = 0,
        /// A URN for artifacts described by URLs.
        /// payload: ArtifactUrlPayload
        Url = 1,
        /// A URN for artifacts embedded in ArtifactInformation proto.
        /// payload: EmbeddedFilePayload.
        Embedded = 2,
        /// A URN for Python artifacts hosted on PYPI.
        /// payload: PypiPayload
        Pypi = 3,
        /// A URN for Java artifacts hosted on a Maven repository.
        /// payload: MavenPayload
        Maven = 4,
        /// A URN for deferred artifacts.
        /// payload: DeferredArtifactPayload
        Deferred = 5,
    }
    impl Types {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Types::File => "FILE",
                Types::Url => "URL",
                Types::Embedded => "EMBEDDED",
                Types::Pypi => "PYPI",
                Types::Maven => "MAVEN",
                Types::Deferred => "DEFERRED",
            }
        }
    }
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
    pub enum Roles {
        /// A URN for staging-to role.
        /// payload: ArtifactStagingToRolePayload
        StagingTo = 0,
        /// A URN for pip-requirements-file role.
        /// payload: None
        PipRequirementsFile = 1,
        /// A URN for the Go worker binary role.
        /// This represents the executable for a Go SDK environment.
        /// A Go environment may have one such artifact with this role.
        /// payload: None
        GoWorkerBinary = 2,
    }
    impl Roles {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Roles::StagingTo => "STAGING_TO",
                Roles::PipRequirementsFile => "PIP_REQUIREMENTS_FILE",
                Roles::GoWorkerBinary => "GO_WORKER_BINARY",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactFilePayload {
    /// a string for an artifact file path e.g. "/tmp/foo.jar"
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// The hex-encoded sha256 checksum of the artifact.
    #[prost(string, tag = "2")]
    pub sha256: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactUrlPayload {
    /// a string for an artifact URL e.g. "<https://.../foo.jar"> or "gs://tmp/foo.jar"
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// (Optional) The hex-encoded sha256 checksum of the artifact if available.
    #[prost(string, tag = "2")]
    pub sha256: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbeddedFilePayload {
    /// raw data bytes for an embedded artifact
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PyPiPayload {
    /// Pypi compatible artifact id e.g. "apache-beam"
    #[prost(string, tag = "1")]
    pub artifact_id: ::prost::alloc::string::String,
    /// Pypi compatible version string.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MavenPayload {
    /// A string specifying Maven artifact.
    /// The standard format is "groupId:artifactId:version\[:packaging[:classifier]\]"
    #[prost(string, tag = "1")]
    pub artifact: ::prost::alloc::string::String,
    /// (Optional) Repository URL. If not specified, Maven central is used by default.
    #[prost(string, tag = "2")]
    pub repository_url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredArtifactPayload {
    /// A unique string identifier assigned by the creator of this payload. The creator may use this key to confirm
    /// whether they can parse the data.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Data for deferred artifacts. Interpretation of bytes is delegated to the creator of this payload.
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactStagingToRolePayload {
    /// A generated staged name (relative path under staging directory).
    #[prost(string, tag = "1")]
    pub staged_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactInformation {
    /// A URN that describes the type of artifact
    #[prost(string, tag = "1")]
    pub type_urn: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub type_payload: ::prost::alloc::vec::Vec<u8>,
    /// A URN that describes the role of artifact
    #[prost(string, tag = "3")]
    pub role_urn: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub role_payload: ::prost::alloc::vec::Vec<u8>,
}
/// An environment for executing UDFs. By default, an SDK container URL, but
/// can also be a process forked by a command, or an externally managed process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// (Required) The URN of the payload
    #[prost(string, tag = "2")]
    pub urn: ::prost::alloc::string::String,
    /// (Optional) The data specifying any parameters to the URN. If
    /// the URN does not require any arguments, this may be omitted.
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// (Optional) Static display data for the environment. If there is none,
    /// it may be omitted.
    #[prost(message, repeated, tag = "4")]
    pub display_data: ::prost::alloc::vec::Vec<DisplayData>,
    /// (Optional) A set of capabilities this environment supports. This is
    /// typically a list of common URNs designating coders, transforms, etc. that
    /// this environment understands (and a runner MAY use) despite not
    /// appearing in the pipeline proto. This may also be used to indicate
    /// support of optional protocols not tied to a concrete component.
    #[prost(string, repeated, tag = "5")]
    pub capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// (Optional) artifact dependency information used for executing UDFs in this environment.
    #[prost(message, repeated, tag = "6")]
    pub dependencies: ::prost::alloc::vec::Vec<ArtifactInformation>,
    /// (Optional) A mapping of resource URNs to requested values.  The encoding
    /// of the values is specified by the URN.  Resource hints are advisory;
    /// a runner is free to ignore resource hints that it does not understand.
    #[prost(map = "string, bytes", tag = "7")]
    pub resource_hints: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardEnvironments {}
/// Nested message and enum types in `StandardEnvironments`.
pub mod standard_environments {
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
    pub enum Environments {
        /// A managed docker container to run user code.
        Docker = 0,
        /// A managed native process to run user code.
        Process = 1,
        /// An external non managed process to run user code.
        External = 2,
        /// Used as a stub when context is missing a runner-provided default environment.
        Default = 3,
    }
    impl Environments {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Environments::Docker => "DOCKER",
                Environments::Process => "PROCESS",
                Environments::External => "EXTERNAL",
                Environments::Default => "DEFAULT",
            }
        }
    }
}
/// The payload of a Docker image
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerPayload {
    /// implicitly linux_amd64.
    #[prost(string, tag = "1")]
    pub container_image: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessPayload {
    /// "linux", "darwin", ..
    #[prost(string, tag = "1")]
    pub os: ::prost::alloc::string::String,
    /// "amd64", ..
    #[prost(string, tag = "2")]
    pub arch: ::prost::alloc::string::String,
    /// process to execute
    #[prost(string, tag = "3")]
    pub command: ::prost::alloc::string::String,
    /// Environment variables
    #[prost(map = "string, string", tag = "4")]
    pub env: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalPayload {
    #[prost(message, optional, tag = "1")]
    pub endpoint: ::core::option::Option<ApiServiceDescriptor>,
    /// Arbitrary extra parameters to pass
    #[prost(map = "string, string", tag = "2")]
    pub params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// These URNs are used to indicate capabilities of environments that cannot
/// simply be expressed as a component (such as a Coder or PTransform) that this
/// environment understands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardProtocols {}
/// Nested message and enum types in `StandardProtocols`.
pub mod standard_protocols {
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
        /// Indicates suport for progress reporting via the legacy Metrics proto.
        LegacyProgressReporting = 0,
        /// Indicates suport for progress reporting via the new MonitoringInfo proto.
        ProgressReporting = 1,
        /// Indicates suport for worker status protocol defined at
        /// <https://s.apache.org/beam-fn-api-harness-status.>
        WorkerStatus = 2,
        /// Indicates this SDK can take advantage of multiple cores when processing
        /// concurrent process bundle requests. (Note that all SDKs must process
        /// an unbounded number of concurrent process bundle requests; this capability
        /// simply indicates this SDK can actually parallelize the work across multiple
        /// cores.
        MultiCoreBundleProcessing = 3,
        /// Indicates this SDK can cheaply spawn sibling workers (e.g. within the
        /// same container) to work around the fact that it cannot take advantage
        /// of multiple cores (i.e. MULTI_CORE_BUNDLE_PROCESSING is not set).
        SiblingWorkers = 5,
        /// Indicates that this SDK handles the InstructionRequest of type
        /// HarnessMonitoringInfosRequest.
        /// A request to provide full MonitoringInfo data associated with
        /// the entire SDK harness process, not specific to a bundle.
        HarnessMonitoringInfos = 4,
        /// Indicates that this SDK can process elements embedded in the
        /// ProcessBundleRequest. See more about the protocol at
        /// <https://s.apache.org/beam-fn-api-control-data-embedding>
        ControlRequestElementsEmbedding = 6,
        /// Indicates that this SDK can cache user state and side inputs across
        /// bundle boundaries. This is a hint to runners that runners can rely on the
        /// SDKs ability to store the data in memory reducing the amount of memory
        /// used overall.
        StateCaching = 7,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::LegacyProgressReporting => "LEGACY_PROGRESS_REPORTING",
                Enum::ProgressReporting => "PROGRESS_REPORTING",
                Enum::WorkerStatus => "WORKER_STATUS",
                Enum::MultiCoreBundleProcessing => "MULTI_CORE_BUNDLE_PROCESSING",
                Enum::SiblingWorkers => "SIBLING_WORKERS",
                Enum::HarnessMonitoringInfos => "HARNESS_MONITORING_INFOS",
                Enum::ControlRequestElementsEmbedding => {
                    "CONTROL_REQUEST_ELEMENTS_EMBEDDING"
                }
                Enum::StateCaching => "STATE_CACHING",
            }
        }
    }
}
/// These URNs are used to indicate capabilities of runner that an environment
/// may take advantage of when interacting with this runner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardRunnerProtocols {}
/// Nested message and enum types in `StandardRunnerProtocols`.
pub mod standard_runner_protocols {
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
        /// Indicates suport the MonitoringInfo short id protocol.
        MonitoringInfoShortIds = 0,
        /// Indicates that this runner can process elements embedded in the
        /// ProcessBundleResponse. See more about the protocol at
        /// <https://s.apache.org/beam-fn-api-control-data-embedding>
        ControlResponseElementsEmbedding = 6,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::MonitoringInfoShortIds => "MONITORING_INFO_SHORT_IDS",
                Enum::ControlResponseElementsEmbedding => {
                    "CONTROL_RESPONSE_ELEMENTS_EMBEDDING"
                }
            }
        }
    }
}
/// These URNs are used to indicate requirements of a pipeline that cannot
/// simply be expressed as a component (such as a Coder or PTransform) that the
/// runner must understand. In many cases, this indicates a particular field
/// of a transform must be inspected and respected (which allows new fields
/// to be added in a forwards-compatible way).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardRequirements {}
/// Nested message and enum types in `StandardRequirements`.
pub mod standard_requirements {
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
        /// This requirement indicates the state_specs and timer_family_specs fields of ParDo
        /// transform payloads must be inspected.
        RequiresStatefulProcessing = 0,
        /// This requirement indicates the requests_finalization field of ParDo
        /// transform payloads must be inspected.
        RequiresBundleFinalization = 1,
        /// This requirement indicates the requires_stable_input field of ParDo
        /// transform payloads must be inspected.
        RequiresStableInput = 2,
        /// This requirement indicates the requires_time_sorted_input field of ParDo
        /// transform payloads must be inspected.
        RequiresTimeSortedInput = 3,
        /// This requirement indicates the restriction_coder_id field of ParDo
        /// transform payloads must be inspected.
        RequiresSplittableDofn = 4,
        /// This requirement indicates that the on_window_expiration_timer_family_spec field
        /// of ParDo transform payloads must be inspected.
        RequiresOnWindowExpiration = 5,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::RequiresStatefulProcessing => "REQUIRES_STATEFUL_PROCESSING",
                Enum::RequiresBundleFinalization => "REQUIRES_BUNDLE_FINALIZATION",
                Enum::RequiresStableInput => "REQUIRES_STABLE_INPUT",
                Enum::RequiresTimeSortedInput => "REQUIRES_TIME_SORTED_INPUT",
                Enum::RequiresSplittableDofn => "REQUIRES_SPLITTABLE_DOFN",
                Enum::RequiresOnWindowExpiration => "REQUIRES_ON_WINDOW_EXPIRATION",
            }
        }
    }
}
/// A URN along with a parameter object whose schema is determined by the
/// URN.
///
/// This structure is reused in two distinct, but compatible, ways:
///
/// 1. This can be a specification of the function over PCollections
///     that a PTransform computes.
/// 2. This can be a specification of a user-defined function, possibly
///     SDK-specific. (external to this message must be adequate context
///     to indicate the environment in which the UDF can be understood).
///
/// Though not explicit in this proto, there are two possibilities
/// for the relationship of a runner to this specification that
/// one should bear in mind:
///
/// 1. The runner understands the URN. For example, it might be
///     a well-known URN like "beam:transform:Top" or
///     "beam:window_fn:FixedWindows" with
///     an agreed-upon payload (e.g. a number or duration,
///     respectively).
/// 2. The runner does not understand the URN. It might be an
///     SDK specific URN such as "beam:dofn:javasdk:1.0"
///     that indicates to the SDK what the payload is,
///     such as a serialized Java DoFn from a particular
///     version of the Beam Java SDK. The payload will often
///     then be an opaque message such as bytes in a
///     language-specific serialization format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionSpec {
    /// (Required) A URN that describes the accompanying payload.
    /// For any URN that is not recognized (by whomever is inspecting
    /// it) the parameter payload should be treated as opaque and
    /// passed as-is.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// (Optional) The data specifying any parameters to the URN. If
    /// the URN does not require any arguments, this may be omitted.
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// A set of well known URNs describing display data.
///
/// All descriptions must contain how the value should be classified and how it
/// is encoded. Note that some types are logical types which convey contextual
/// information about the pipeline in addition to an encoding while others only
/// specify the encoding itself.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardDisplayData {}
/// Nested message and enum types in `StandardDisplayData`.
pub mod standard_display_data {
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
    pub enum DisplayData {
        /// A string label and value. Has a payload containing an encoded
        /// LabelledPayload.
        Labelled = 0,
    }
    impl DisplayData {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DisplayData::Labelled => "LABELLED",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelledPayload {
    /// (Required) A human readable label for the value.
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// (Required) The key identifies the actual content of the metadata.
    #[prost(string, tag = "6")]
    pub key: ::prost::alloc::string::String,
    /// (Required) The namespace describes the context that specified the key.
    #[prost(string, tag = "7")]
    pub namespace: ::prost::alloc::string::String,
    /// (Required) A value which will be displayed to the user.
    #[prost(oneof = "labelled_payload::Value", tags = "2, 3, 4, 5")]
    pub value: ::core::option::Option<labelled_payload::Value>,
}
/// Nested message and enum types in `LabelledPayload`.
pub mod labelled_payload {
    /// (Required) A value which will be displayed to the user.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "2")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag = "3")]
        BoolValue(bool),
        #[prost(double, tag = "4")]
        DoubleValue(f64),
        #[prost(int64, tag = "5")]
        IntValue(i64),
    }
}
/// Static display data associated with a pipeline component. Display data is
/// useful for pipeline runners IOs and diagnostic dashboards to display details
/// about annotated components.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayData {
    /// A key used to describe the type of display data. See StandardDisplayData
    /// for the set of well known urns describing how the payload is meant to be
    /// interpreted.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// (Optional) The data specifying any parameters to the URN. If
    /// the URN does not require any arguments, this may be omitted.
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// A disjoint union of all the things that may contain references
/// that require Components to resolve.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageWithComponents {
    /// (Optional) The by-reference components of the root message,
    /// enabling a standalone message.
    ///
    /// If this is absent, it is expected that there are no
    /// references.
    #[prost(message, optional, tag = "1")]
    pub components: ::core::option::Option<Components>,
    /// (Required) The root message that may contain pointers
    /// that should be resolved by looking inside components.
    #[prost(
        oneof = "message_with_components::Root",
        tags = "2, 3, 4, 6, 7, 8, 9, 11, 12, 13"
    )]
    pub root: ::core::option::Option<message_with_components::Root>,
}
/// Nested message and enum types in `MessageWithComponents`.
pub mod message_with_components {
    /// (Required) The root message that may contain pointers
    /// that should be resolved by looking inside components.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Root {
        #[prost(message, tag = "2")]
        Coder(super::Coder),
        #[prost(message, tag = "3")]
        CombinePayload(super::CombinePayload),
        #[prost(message, tag = "4")]
        FunctionSpec(super::FunctionSpec),
        #[prost(message, tag = "6")]
        ParDoPayload(super::ParDoPayload),
        #[prost(message, tag = "7")]
        Ptransform(super::PTransform),
        #[prost(message, tag = "8")]
        Pcollection(super::PCollection),
        #[prost(message, tag = "9")]
        ReadPayload(super::ReadPayload),
        #[prost(message, tag = "11")]
        SideInput(super::SideInput),
        #[prost(message, tag = "12")]
        WindowIntoPayload(super::WindowIntoPayload),
        #[prost(message, tag = "13")]
        WindowingStrategy(super::WindowingStrategy),
    }
}
/// The payload for an executable stage. This will eventually be passed to an SDK in the form of a
/// ProcessBundleDescriptor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutableStagePayload {
    /// (Required) Environment in which this stage executes.
    ///
    /// We use an environment rather than environment id
    /// because ExecutableStages use environments directly. This may change in the future.
    #[prost(message, optional, tag = "1")]
    pub environment: ::core::option::Option<Environment>,
    /// The wire coder settings of this executable stage
    #[prost(message, repeated, tag = "9")]
    pub wire_coder_settings: ::prost::alloc::vec::Vec<
        executable_stage_payload::WireCoderSetting,
    >,
    /// (Required) Input PCollection id. This must be present as a value in the inputs of any
    /// PTransform the ExecutableStagePayload is the payload of.
    #[prost(string, tag = "2")]
    pub input: ::prost::alloc::string::String,
    /// The side inputs required for this executable stage. Each side input of each PTransform within
    /// this ExecutableStagePayload must be represented within this field.
    #[prost(message, repeated, tag = "3")]
    pub side_inputs: ::prost::alloc::vec::Vec<executable_stage_payload::SideInputId>,
    /// PTransform ids contained within this executable stage. This must contain at least one
    /// PTransform id.
    #[prost(string, repeated, tag = "4")]
    pub transforms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output PCollection ids. This must be equal to the values of the outputs of any
    /// PTransform the ExecutableStagePayload is the payload of.
    #[prost(string, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// (Required) The components for the Executable Stage. This must contain all of the Transforms
    /// in transforms, and the closure of all of the components they recognize.
    #[prost(message, optional, tag = "6")]
    pub components: ::core::option::Option<Components>,
    /// The user states required for this executable stage. Each user state of each PTransform within
    /// this ExecutableStagePayload must be represented within this field.
    #[prost(message, repeated, tag = "7")]
    pub user_states: ::prost::alloc::vec::Vec<executable_stage_payload::UserStateId>,
    /// The timers required for this executable stage. Each timer of each PTransform within
    /// this ExecutableStagePayload must be represented within this field.
    #[prost(message, repeated, tag = "8")]
    pub timers: ::prost::alloc::vec::Vec<executable_stage_payload::TimerId>,
    /// The timerfamilies required for this executable stage. Each timer familyof each PTransform within
    /// this ExecutableStagePayload must be represented within this field.
    #[prost(message, repeated, tag = "10")]
    pub timer_families: ::prost::alloc::vec::Vec<
        executable_stage_payload::TimerFamilyId,
    >,
}
/// Nested message and enum types in `ExecutableStagePayload`.
pub mod executable_stage_payload {
    /// A reference to a side input. Side inputs are uniquely identified by PTransform id and
    /// local name.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SideInputId {
        /// (Required) The id of the PTransform that references this side input.
        #[prost(string, tag = "1")]
        pub transform_id: ::prost::alloc::string::String,
        /// (Required) The local name of this side input from the PTransform that references it.
        #[prost(string, tag = "2")]
        pub local_name: ::prost::alloc::string::String,
    }
    /// A reference to user state. User states are uniquely identified by PTransform id and
    /// local name.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserStateId {
        /// (Required) The id of the PTransform that references this user state.
        #[prost(string, tag = "1")]
        pub transform_id: ::prost::alloc::string::String,
        /// (Required) The local name of this user state for the PTransform that references it.
        #[prost(string, tag = "2")]
        pub local_name: ::prost::alloc::string::String,
    }
    /// A reference to a timer. Timers are uniquely identified by PTransform id and
    /// local name.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimerId {
        /// (Required) The id of the PTransform that references this timer.
        #[prost(string, tag = "1")]
        pub transform_id: ::prost::alloc::string::String,
        /// (Required) The local name of this timer for the PTransform that references it.
        #[prost(string, tag = "2")]
        pub local_name: ::prost::alloc::string::String,
    }
    /// A reference to a timer. Timers are uniquely identified by PTransform id and
    /// local name.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimerFamilyId {
        /// (Required) The id of the PTransform that references this timer family.
        #[prost(string, tag = "1")]
        pub transform_id: ::prost::alloc::string::String,
        /// (Required) The local name of this timer family for the PTransform that references it.
        #[prost(string, tag = "2")]
        pub local_name: ::prost::alloc::string::String,
    }
    /// Settings that decide the coder type of wire coder.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WireCoderSetting {
        /// (Required) The URN of the wire coder.
        /// Note that only windowed value coder or parameterized windowed value coder are supported.
        #[prost(string, tag = "1")]
        pub urn: ::prost::alloc::string::String,
        /// (Optional) The data specifying any parameters to the URN. If
        /// the URN is beam:coder:windowed_value:v1, this may be omitted. If the URN is
        /// beam:coder:param_windowed_value:v1, the payload is an encoded windowed
        /// value using the beam:coder:windowed_value:v1 coder parameterized by
        /// a beam:coder:bytes:v1 element coder and the window coder that this
        /// param_windowed_value coder uses.
        #[prost(bytes = "vec", tag = "2")]
        pub payload: ::prost::alloc::vec::Vec<u8>,
        /// (Required) The target(PCollection or Timer) this setting applies to.
        #[prost(oneof = "wire_coder_setting::Target", tags = "3, 4")]
        pub target: ::core::option::Option<wire_coder_setting::Target>,
    }
    /// Nested message and enum types in `WireCoderSetting`.
    pub mod wire_coder_setting {
        /// (Required) The target(PCollection or Timer) this setting applies to.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Target {
            /// The input or output PCollection id this setting applies to.
            #[prost(string, tag = "3")]
            InputOrOutputId(::prost::alloc::string::String),
            /// The timer id this setting applies to.
            #[prost(message, tag = "4")]
            Timer(super::TimerId),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardResourceHints {}
/// Nested message and enum types in `StandardResourceHints`.
pub mod standard_resource_hints {
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
        /// Describes hardware accelerators that are desired to have in the execution environment.
        Accelerator = 0,
        /// Describes desired minimal available RAM size in transform's execution environment.
        /// SDKs should convert the size to bytes, but can allow users to specify human-friendly units (e.g. GiB).
        MinRamBytes = 1,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Accelerator => "ACCELERATOR",
                Enum::MinRamBytes => "MIN_RAM_BYTES",
            }
        }
    }
}
/// Generated client implementations.
pub mod test_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TestStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TestStreamServiceClient<tonic::transport::Channel> {
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
    impl<T> TestStreamServiceClient<T>
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
        ) -> TestStreamServiceClient<InterceptedService<T, F>>
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
            TestStreamServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// A TestStream will request for events using this RPC.
        pub async fn events(
            &mut self,
            request: impl tonic::IntoRequest<super::EventsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::test_stream_payload::Event>>,
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
                "/org.apache.beam.model.pipeline.v1.TestStreamService/Events",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod test_stream_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with TestStreamServiceServer.
    #[async_trait]
    pub trait TestStreamService: Send + Sync + 'static {
        ///Server streaming response type for the Events method.
        type EventsStream: futures_core::Stream<
                Item = Result<super::test_stream_payload::Event, tonic::Status>,
            >
            + Send
            + 'static;
        /// A TestStream will request for events using this RPC.
        async fn events(
            &self,
            request: tonic::Request<super::EventsRequest>,
        ) -> Result<tonic::Response<Self::EventsStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TestStreamServiceServer<T: TestStreamService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TestStreamService> TestStreamServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TestStreamServiceServer<T>
    where
        T: TestStreamService,
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
                "/org.apache.beam.model.pipeline.v1.TestStreamService/Events" => {
                    #[allow(non_camel_case_types)]
                    struct EventsSvc<T: TestStreamService>(pub Arc<T>);
                    impl<
                        T: TestStreamService,
                    > tonic::server::ServerStreamingService<super::EventsRequest>
                    for EventsSvc<T> {
                        type Response = super::test_stream_payload::Event;
                        type ResponseStream = T::EventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EventsSvc(inner);
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
    impl<T: TestStreamService> Clone for TestStreamServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TestStreamService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TestStreamService> tonic::server::NamedService
    for TestStreamServiceServer<T> {
        const NAME: &'static str = "org.apache.beam.model.pipeline.v1.TestStreamService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// List of fields for this schema. Two fields may not share a name.
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    /// REQUIRED. An RFC 4122 UUID.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// Indicates that encoding positions have been overridden.
    #[prost(bool, tag = "4")]
    pub encoding_positions_set: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    /// REQUIRED. Name of this field within the schema.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// OPTIONAL. Human readable description of this field, such as the query that generated it.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub r#type: ::core::option::Option<FieldType>,
    #[prost(int32, tag = "4")]
    pub id: i32,
    /// OPTIONAL. The position of this field's data when encoded, e.g. with beam:coder:row:v1.
    /// Either no fields in a given row are have encoding position populated,
    /// or all of them are. Used to support backwards compatibility with schema
    /// changes.
    /// If no fields have encoding position populated the order of encoding is the same as the order in the Schema.
    /// If this Field is part of a Schema where encoding_positions_set is True then encoding_position must be
    /// defined, otherwise this field is ignored.
    #[prost(int32, tag = "5")]
    pub encoding_position: i32,
    #[prost(message, repeated, tag = "6")]
    pub options: ::prost::alloc::vec::Vec<Option>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldType {
    #[prost(bool, tag = "1")]
    pub nullable: bool,
    #[prost(oneof = "field_type::TypeInfo", tags = "2, 3, 4, 5, 6, 7")]
    pub type_info: ::core::option::Option<field_type::TypeInfo>,
}
/// Nested message and enum types in `FieldType`.
pub mod field_type {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeInfo {
        #[prost(enumeration = "super::AtomicType", tag = "2")]
        AtomicType(i32),
        #[prost(message, tag = "3")]
        ArrayType(::prost::alloc::boxed::Box<super::ArrayType>),
        #[prost(message, tag = "4")]
        IterableType(::prost::alloc::boxed::Box<super::IterableType>),
        #[prost(message, tag = "5")]
        MapType(::prost::alloc::boxed::Box<super::MapType>),
        #[prost(message, tag = "6")]
        RowType(super::RowType),
        #[prost(message, tag = "7")]
        LogicalType(::prost::alloc::boxed::Box<super::LogicalType>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayType {
    #[prost(message, optional, boxed, tag = "1")]
    pub element_type: ::core::option::Option<::prost::alloc::boxed::Box<FieldType>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IterableType {
    #[prost(message, optional, boxed, tag = "1")]
    pub element_type: ::core::option::Option<::prost::alloc::boxed::Box<FieldType>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapType {
    #[prost(message, optional, boxed, tag = "1")]
    pub key_type: ::core::option::Option<::prost::alloc::boxed::Box<FieldType>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub value_type: ::core::option::Option<::prost::alloc::boxed::Box<FieldType>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowType {
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<Schema>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalType {
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, boxed, tag = "3")]
    pub representation: ::core::option::Option<::prost::alloc::boxed::Box<FieldType>>,
    #[prost(message, optional, boxed, tag = "4")]
    pub argument_type: ::core::option::Option<::prost::alloc::boxed::Box<FieldType>>,
    #[prost(message, optional, tag = "5")]
    pub argument: ::core::option::Option<FieldValue>,
}
/// Universally defined Logical types for Row schemas.
/// These logical types are supposed to be understood by all SDKs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalTypes {}
/// Nested message and enum types in `LogicalTypes`.
pub mod logical_types {
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
        /// A URN for Python Callable logical type
        ///    - Representation type: STRING
        ///    - Language type: In Python SDK, PythonCallableWithSource.
        ///      In any other SDKs, a wrapper object for a string which
        ///      can be evaluated to a Python Callable object.
        PythonCallable = 0,
        /// A URN for MicrosInstant type
        ///    - Representation type: ROW<seconds: INT64, micros: INT64>
        ///    - A timestamp without a timezone where seconds + micros represents the
        ///      amount of time since the epoch.
        MicrosInstant = 1,
        /// A URN for MillisInstant type
        ///    - Representation type: INT64
        ///    - A timestamp without a timezone represented by the number of
        ///      milliseconds since the epoch. The INT64 value is encoded with
        ///      big-endian shifted such that lexicographic ordering of the bytes
        ///      corresponds to chronological order.
        MillisInstant = 2,
        /// A URN for Decimal type
        ///    - Representation type: BYTES
        ///    - A decimal number with variable scale. Its BYTES
        ///      representation consists of an integer (INT32) scale followed by a
        ///      two's complement encoded big integer.
        Decimal = 3,
        /// A URN for FixedLengthBytes type
        ///    - Representation type: BYTES
        ///    - Argument type: INT32.
        ///      A fixed-length bytes with its length as the argument.
        FixedBytes = 4,
        /// A URN for VariableLengthBytes type
        ///    - Representation type: BYTES
        ///    - Argument type: INT32.
        ///      A variable-length bytes with its maximum length as the argument.
        VarBytes = 5,
        /// A URN for FixedLengthString type
        ///    - Representation type: STRING
        ///    - Argument type: INT32.
        ///      A fixed-length string with its length as the argument.
        FixedChar = 6,
        /// A URN for VariableLengthString type
        ///    - Representation type: STRING
        ///    - Argument type: INT32.
        ///      A variable-length string with its maximum length as the argument.
        VarChar = 7,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::PythonCallable => "PYTHON_CALLABLE",
                Enum::MicrosInstant => "MICROS_INSTANT",
                Enum::MillisInstant => "MILLIS_INSTANT",
                Enum::Decimal => "DECIMAL",
                Enum::FixedBytes => "FIXED_BYTES",
                Enum::VarBytes => "VAR_BYTES",
                Enum::FixedChar => "FIXED_CHAR",
                Enum::VarChar => "VAR_CHAR",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    /// REQUIRED. Identifier for the option.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// REQUIRED. Type specifier for the structure of value.
    /// Conventionally, options that don't require additional configuration should
    /// use a boolean type, with the value set to true.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<FieldType>,
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<FieldValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<FieldValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldValue {
    /// If none of these are set, value is considered null.
    #[prost(oneof = "field_value::FieldValue", tags = "1, 2, 3, 4, 5, 6")]
    pub field_value: ::core::option::Option<field_value::FieldValue>,
}
/// Nested message and enum types in `FieldValue`.
pub mod field_value {
    /// If none of these are set, value is considered null.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FieldValue {
        #[prost(message, tag = "1")]
        AtomicValue(super::AtomicTypeValue),
        #[prost(message, tag = "2")]
        ArrayValue(super::ArrayTypeValue),
        #[prost(message, tag = "3")]
        IterableValue(super::IterableTypeValue),
        #[prost(message, tag = "4")]
        MapValue(super::MapTypeValue),
        #[prost(message, tag = "5")]
        RowValue(super::Row),
        #[prost(message, tag = "6")]
        LogicalTypeValue(::prost::alloc::boxed::Box<super::LogicalTypeValue>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtomicTypeValue {
    #[prost(oneof = "atomic_type_value::Value", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub value: ::core::option::Option<atomic_type_value::Value>,
}
/// Nested message and enum types in `AtomicTypeValue`.
pub mod atomic_type_value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(int32, tag = "1")]
        Byte(i32),
        #[prost(int32, tag = "2")]
        Int16(i32),
        #[prost(int32, tag = "3")]
        Int32(i32),
        #[prost(int64, tag = "4")]
        Int64(i64),
        #[prost(float, tag = "5")]
        Float(f32),
        #[prost(double, tag = "6")]
        Double(f64),
        #[prost(string, tag = "7")]
        String(::prost::alloc::string::String),
        #[prost(bool, tag = "8")]
        Boolean(bool),
        #[prost(bytes, tag = "9")]
        Bytes(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayTypeValue {
    #[prost(message, repeated, tag = "1")]
    pub element: ::prost::alloc::vec::Vec<FieldValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IterableTypeValue {
    #[prost(message, repeated, tag = "1")]
    pub element: ::prost::alloc::vec::Vec<FieldValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapTypeValue {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<MapTypeEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapTypeEntry {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<FieldValue>,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<FieldValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalTypeValue {
    #[prost(message, optional, boxed, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::boxed::Box<FieldValue>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AtomicType {
    Unspecified = 0,
    Byte = 1,
    Int16 = 2,
    Int32 = 3,
    Int64 = 4,
    Float = 5,
    Double = 6,
    String = 7,
    Boolean = 8,
    Bytes = 9,
}
impl AtomicType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AtomicType::Unspecified => "UNSPECIFIED",
            AtomicType::Byte => "BYTE",
            AtomicType::Int16 => "INT16",
            AtomicType::Int32 => "INT32",
            AtomicType::Int64 => "INT64",
            AtomicType::Float => "FLOAT",
            AtomicType::Double => "DOUBLE",
            AtomicType::String => "STRING",
            AtomicType::Boolean => "BOOLEAN",
            AtomicType::Bytes => "BYTES",
        }
    }
}
/// A configuration payload for an external transform.
/// Used as the payload of ExternalTransform as part of an ExpansionRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalConfigurationPayload {
    /// A schema for use in beam:coder:row:v1
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<Schema>,
    /// A payload which can be decoded using beam:coder:row:v1 and the given
    /// schema.
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// Defines specific expansion methods that may be used to expand cross-language
/// transforms.
/// Has to be set as the URN of the transform of the expansion request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpansionMethods {}
/// Nested message and enum types in `ExpansionMethods`.
pub mod expansion_methods {
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
        /// Expand a Java transform using specified constructor and builder methods.
        /// Transform payload will be of type JavaClassLookupPayload.
        JavaClassLookup = 0,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::JavaClassLookup => "JAVA_CLASS_LOOKUP",
            }
        }
    }
}
/// A configuration payload for an external transform.
/// Used to define a Java transform that can be directly instantiated by a Java
/// expansion service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JavaClassLookupPayload {
    /// Name of the Java transform class.
    #[prost(string, tag = "1")]
    pub class_name: ::prost::alloc::string::String,
    /// A static method to construct the initial instance of the transform.
    /// If not provided, the transform should be instantiated using a class
    /// constructor.
    #[prost(string, tag = "2")]
    pub constructor_method: ::prost::alloc::string::String,
    /// The top level fields of the schema represent the method parameters in
    /// order.
    /// If able, top level field names are also verified against the method
    /// parameters for a match.
    /// Any field names in the form 'ignore\[0-9\]+' will not be used for validation
    /// hence that format can be used to represent arbitrary field names.
    #[prost(message, optional, tag = "3")]
    pub constructor_schema: ::core::option::Option<Schema>,
    /// A payload which can be decoded using beam:coder:row:v1 and the provided
    /// constructor schema.
    #[prost(bytes = "vec", tag = "4")]
    pub constructor_payload: ::prost::alloc::vec::Vec<u8>,
    /// Set of builder methods and corresponding parameters to apply after the
    /// transform object is constructed.
    /// When constructing the transform object, given builder methods will be
    /// applied in order.
    #[prost(message, repeated, tag = "5")]
    pub builder_methods: ::prost::alloc::vec::Vec<BuilderMethod>,
}
/// This represents a builder method of the transform class that should be
/// applied in-order after instantiating the initial transform object.
/// Each builder method may take one or more parameters and has to return an
/// instance of the transform object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuilderMethod {
    /// Name of the builder method
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The top level fields of the schema represent the method parameters in
    /// order.
    /// If able, top level field names are also verified against the method
    /// parameters for a match.
    /// Any field names in the form 'ignore\[0-9\]+' will not be used for validation
    /// hence that format can be used to represent arbitrary field names.
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<Schema>,
    /// A payload which can be decoded using beam:coder:row:v1 and the builder
    /// method schema.
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// A specification for describing a well known MonitoringInfo.
///
/// All specifications are uniquely identified by the urn.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoringInfoSpec {
    /// Defines the semantic meaning of the metric or monitored state.
    ///
    /// See MonitoringInfoSpecs.Enum for the set of well known metrics/monitored
    /// state.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// Defines the required encoding and aggregation method for the payload.
    ///
    /// See MonitoringInfoTypeUrns.Enum for the set of well known types.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// The list of required labels for the specified urn and type.
    #[prost(string, repeated, tag = "3")]
    pub required_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Extra non functional parts of the spec for descriptive purposes.
    /// i.e. description, units, etc.
    #[prost(message, repeated, tag = "4")]
    pub annotations: ::prost::alloc::vec::Vec<Annotation>,
}
/// The key name and value string of MonitoringInfo annotations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// A set of well known MonitoringInfo specifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoringInfoSpecs {}
/// Nested message and enum types in `MonitoringInfoSpecs`.
pub mod monitoring_info_specs {
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
        /// Represents an integer counter where values are summed across bundles.
        UserSumInt64 = 0,
        /// Represents a double counter where values are summed across bundles.
        UserSumDouble = 1,
        /// Represents a distribution of an integer value where:
        ///    - count: represents the number of values seen across all bundles
        ///    - sum: represents the total of the value across all bundles
        ///    - min: represents the smallest value seen across all bundles
        ///    - max: represents the largest value seen across all bundles
        UserDistributionInt64 = 2,
        /// Represents a distribution of a double value where:
        ///    - count: represents the number of values seen across all bundles
        ///    - sum: represents the total of the value across all bundles
        ///    - min: represents the smallest value seen across all bundles
        ///    - max: represents the largest value seen across all bundles
        UserDistributionDouble = 3,
        /// Represents the latest seen integer value. The timestamp is used to
        /// provide an "ordering" over multiple values to determine which is the
        /// latest.
        UserLatestInt64 = 4,
        /// Represents the latest seen double value. The timestamp is used to
        /// provide an "ordering" over multiple values to determine which is the
        /// latest.
        UserLatestDouble = 5,
        /// Represents the largest set of integer values seen across bundles.
        UserTopNInt64 = 6,
        /// Represents the largest set of double values seen across bundles.
        UserTopNDouble = 7,
        /// Represents the smallest set of integer values seen across bundles.
        UserBottomNInt64 = 8,
        /// Represents the smallest set of double values seen across bundles.
        UserBottomNDouble = 9,
        ElementCount = 10,
        SampledByteSize = 11,
        StartBundleMsecs = 12,
        ProcessBundleMsecs = 13,
        FinishBundleMsecs = 14,
        TotalMsecs = 15,
        /// All values reported across all beam:metric:ptransform_progress:.*:v1
        /// metrics are of the same magnitude.
        WorkRemaining = 16,
        /// All values reported across all beam:metric:ptransform_progress:.*:v1
        /// metrics are of the same magnitude.
        WorkCompleted = 17,
        /// The (0-based) index of the latest item processed from the data channel.
        /// This gives an indication of the SDKs progress through the data channel,
        /// and is a lower bound on where it is able to split.
        /// For an SDK that processes items sequentially, this is equivalently the
        /// number of items fully processed (or -1 if processing has not yet started).
        DataChannelReadIndex = 18,
        ApiRequestCount = 19,
        ApiRequestLatencies = 20,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::UserSumInt64 => "USER_SUM_INT64",
                Enum::UserSumDouble => "USER_SUM_DOUBLE",
                Enum::UserDistributionInt64 => "USER_DISTRIBUTION_INT64",
                Enum::UserDistributionDouble => "USER_DISTRIBUTION_DOUBLE",
                Enum::UserLatestInt64 => "USER_LATEST_INT64",
                Enum::UserLatestDouble => "USER_LATEST_DOUBLE",
                Enum::UserTopNInt64 => "USER_TOP_N_INT64",
                Enum::UserTopNDouble => "USER_TOP_N_DOUBLE",
                Enum::UserBottomNInt64 => "USER_BOTTOM_N_INT64",
                Enum::UserBottomNDouble => "USER_BOTTOM_N_DOUBLE",
                Enum::ElementCount => "ELEMENT_COUNT",
                Enum::SampledByteSize => "SAMPLED_BYTE_SIZE",
                Enum::StartBundleMsecs => "START_BUNDLE_MSECS",
                Enum::ProcessBundleMsecs => "PROCESS_BUNDLE_MSECS",
                Enum::FinishBundleMsecs => "FINISH_BUNDLE_MSECS",
                Enum::TotalMsecs => "TOTAL_MSECS",
                Enum::WorkRemaining => "WORK_REMAINING",
                Enum::WorkCompleted => "WORK_COMPLETED",
                Enum::DataChannelReadIndex => "DATA_CHANNEL_READ_INDEX",
                Enum::ApiRequestCount => "API_REQUEST_COUNT",
                Enum::ApiRequestLatencies => "API_REQUEST_LATENCIES",
            }
        }
    }
}
/// A set of properties for the MonitoringInfoLabel, this is useful to obtain
/// the proper label string for the MonitoringInfoLabel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoringInfoLabelProps {
    /// The label key to use in the MonitoringInfo labels map.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoringInfo {
    /// (Required) Defines the semantic meaning of the metric or monitored state.
    ///
    /// See MonitoringInfoSpecs.Enum for the set of well known metrics/monitored
    /// state.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// (Required) Defines the encoding and aggregation method for the payload.
    ///
    /// See MonitoringInfoTypeUrns.Enum for the set of well known types.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// (Required) The metric or monitored state encoded as per the specification
    /// defined by the type.
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// A set of key and value labels which define the scope of the metric. For
    /// well known URNs, the set of required labels is provided by the associated
    /// MonitoringInfoSpec.
    ///
    /// Either a well defined entity id for matching the enum names in
    /// the MonitoringInfoLabels enum or any arbitrary label
    /// set by a custom metric or user metric.
    ///
    /// A monitoring system is expected to be able to aggregate the metrics
    /// together for all updates having the same URN and labels. Some systems such
    /// as Stackdriver will be able to aggregate the metrics using a subset of the
    /// provided labels
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// This indicates the start of the time range over which this value was
    /// measured.
    /// This is needed by some external metric aggregation services
    /// to indicate when the reporter of the metric first began collecting the
    /// cumulative value for the timeseries.
    /// If the SDK Harness restarts, it should reset the start_time, and reset
    /// the collection of cumulative metrics (i.e. start to count again from 0).
    /// HarnessMonitoringInfos should set this start_time once, when the
    /// MonitoringInfo is first reported.
    /// ProcessBundle MonitoringInfos should set a start_time for each bundle.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `MonitoringInfo`.
pub mod monitoring_info {
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
    pub enum MonitoringInfoLabels {
        /// The values used for TRANSFORM, PCOLLECTION, WINDOWING_STRATEGY
        /// CODER, ENVIRONMENT, etc. must always match the keys used to
        /// refer to them. For actively processed bundles, these should match the
        /// values within the ProcessBundleDescriptor. For job management APIs,
        /// these should match values within the original pipeline representation.
        Transform = 0,
        Pcollection = 1,
        WindowingStrategy = 2,
        Coder = 3,
        Environment = 4,
        Namespace = 5,
        Name = 6,
        Service = 7,
        Method = 8,
        Resource = 9,
        Status = 10,
        BigqueryProjectId = 11,
        BigqueryDataset = 12,
        BigqueryTable = 13,
        BigqueryView = 14,
        BigqueryQueryName = 15,
        GcsBucket = 16,
        GcsProjectId = 17,
        DatastoreProject = 18,
        DatastoreNamespace = 19,
        BigtableProjectId = 20,
        InstanceId = 21,
        TableId = 22,
        SpannerProjectId = 23,
        SpannerDatabaseId = 24,
        SpannerTableId = 25,
        SpannerInstanceId = 26,
        SpannerQueryName = 27,
    }
    impl MonitoringInfoLabels {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MonitoringInfoLabels::Transform => "TRANSFORM",
                MonitoringInfoLabels::Pcollection => "PCOLLECTION",
                MonitoringInfoLabels::WindowingStrategy => "WINDOWING_STRATEGY",
                MonitoringInfoLabels::Coder => "CODER",
                MonitoringInfoLabels::Environment => "ENVIRONMENT",
                MonitoringInfoLabels::Namespace => "NAMESPACE",
                MonitoringInfoLabels::Name => "NAME",
                MonitoringInfoLabels::Service => "SERVICE",
                MonitoringInfoLabels::Method => "METHOD",
                MonitoringInfoLabels::Resource => "RESOURCE",
                MonitoringInfoLabels::Status => "STATUS",
                MonitoringInfoLabels::BigqueryProjectId => "BIGQUERY_PROJECT_ID",
                MonitoringInfoLabels::BigqueryDataset => "BIGQUERY_DATASET",
                MonitoringInfoLabels::BigqueryTable => "BIGQUERY_TABLE",
                MonitoringInfoLabels::BigqueryView => "BIGQUERY_VIEW",
                MonitoringInfoLabels::BigqueryQueryName => "BIGQUERY_QUERY_NAME",
                MonitoringInfoLabels::GcsBucket => "GCS_BUCKET",
                MonitoringInfoLabels::GcsProjectId => "GCS_PROJECT_ID",
                MonitoringInfoLabels::DatastoreProject => "DATASTORE_PROJECT",
                MonitoringInfoLabels::DatastoreNamespace => "DATASTORE_NAMESPACE",
                MonitoringInfoLabels::BigtableProjectId => "BIGTABLE_PROJECT_ID",
                MonitoringInfoLabels::InstanceId => "INSTANCE_ID",
                MonitoringInfoLabels::TableId => "TABLE_ID",
                MonitoringInfoLabels::SpannerProjectId => "SPANNER_PROJECT_ID",
                MonitoringInfoLabels::SpannerDatabaseId => "SPANNER_DATABASE_ID",
                MonitoringInfoLabels::SpannerTableId => "SPANNER_TABLE_ID",
                MonitoringInfoLabels::SpannerInstanceId => "SPANNER_INSTANCE_ID",
                MonitoringInfoLabels::SpannerQueryName => "SPANNER_QUERY_NAME",
            }
        }
    }
}
/// A set of well known URNs that specify the encoding and aggregation method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoringInfoTypeUrns {}
/// Nested message and enum types in `MonitoringInfoTypeUrns`.
pub mod monitoring_info_type_urns {
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
        /// Represents an integer counter where values are summed across bundles.
        ///
        /// Encoding: <value>
        ///    - value: beam:coder:varint:v1
        SumInt64Type = 0,
        /// Represents a double counter where values are summed across bundles.
        ///
        /// Encoding: <value>
        ///    value: beam:coder:double:v1
        SumDoubleType = 1,
        /// Represents a distribution of an integer value where:
        ///    - count: represents the number of values seen across all bundles
        ///    - sum: represents the total of the value across all bundles
        ///    - min: represents the smallest value seen across all bundles
        ///    - max: represents the largest value seen across all bundles
        ///
        /// Encoding: <count><sum><min><max>
        ///    - count: beam:coder:varint:v1
        ///    - sum:   beam:coder:varint:v1
        ///    - min:   beam:coder:varint:v1
        ///    - max:   beam:coder:varint:v1
        DistributionInt64Type = 2,
        /// Represents a distribution of a double value where:
        ///    - count: represents the number of values seen across all bundles
        ///    - sum: represents the total of the value across all bundles
        ///    - min: represents the smallest value seen across all bundles
        ///    - max: represents the largest value seen across all bundles
        ///
        /// Encoding: <count><sum><min><max>
        ///    - count: beam:coder:varint:v1
        ///    - sum:   beam:coder:double:v1
        ///    - min:   beam:coder:double:v1
        ///    - max:   beam:coder:double:v1
        DistributionDoubleType = 3,
        /// Represents the latest seen integer value. The timestamp is used to
        /// provide an "ordering" over multiple values to determine which is the
        /// latest.
        ///
        /// Encoding: <timestamp><value>
        ///    - timestamp: beam:coder:varint:v1     (milliseconds since epoch)
        ///    - value:     beam:coder:varint:v1
        LatestInt64Type = 4,
        /// Represents the latest seen double value. The timestamp is used to
        /// provide an "ordering" over multiple values to determine which is the
        /// latest.
        ///
        /// Encoding: <timestamp><value>
        ///    - timestamp: beam:coder:varint:v1     (milliseconds since epoch)
        ///    - value:     beam:coder:double:v1
        LatestDoubleType = 5,
        /// Represents the largest set of integer values seen across bundles.
        ///
        /// Encoding: <iter><value1><value2>...<valueN></iter>
        ///    - iter:   beam:coder:iterable:v1
        ///    - valueX: beam:coder:varint:v1
        TopNInt64Type = 6,
        /// Represents the largest set of double values seen across bundles.
        ///
        /// Encoding: <iter><value1><value2>...<valueN></iter>
        ///    - iter:   beam:coder:iterable:v1
        ///    - valueX: beam:coder<beam:coder:double:v1
        TopNDoubleType = 7,
        /// Represents the smallest set of integer values seen across bundles.
        ///
        /// Encoding: <iter><value1><value2>...<valueN></iter>
        ///    - iter:   beam:coder:iterable:v1
        ///    - valueX: beam:coder:varint:v1
        BottomNInt64Type = 8,
        /// Represents the smallest set of double values seen across bundles.
        ///
        /// Encoding: <iter><value1><value2>...<valueN></iter>
        ///    - iter:   beam:coder:iterable:v1
        ///    - valueX: beam:coder:double:v1
        BottomNDoubleType = 9,
        /// Encoding: <iter><value1><value2>...<valueN></iter>
        ///    - iter:   beam:coder:iterable:v1
        ///    - valueX: beam:coder:double:v1
        ProgressType = 10,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::SumInt64Type => "SUM_INT64_TYPE",
                Enum::SumDoubleType => "SUM_DOUBLE_TYPE",
                Enum::DistributionInt64Type => "DISTRIBUTION_INT64_TYPE",
                Enum::DistributionDoubleType => "DISTRIBUTION_DOUBLE_TYPE",
                Enum::LatestInt64Type => "LATEST_INT64_TYPE",
                Enum::LatestDoubleType => "LATEST_DOUBLE_TYPE",
                Enum::TopNInt64Type => "TOP_N_INT64_TYPE",
                Enum::TopNDoubleType => "TOP_N_DOUBLE_TYPE",
                Enum::BottomNInt64Type => "BOTTOM_N_INT64_TYPE",
                Enum::BottomNDoubleType => "BOTTOM_N_DOUBLE_TYPE",
                Enum::ProgressType => "PROGRESS_TYPE",
            }
        }
    }
}
/// By default, all data in a PCollection is assigned to the single global
/// window. See BeamConstants for the time span this window encompasses.
///
/// See <https://beam.apache.org/documentation/programming-guide/#single-global-window>
/// for additional details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalWindowsPayload {}
/// Nested message and enum types in `GlobalWindowsPayload`.
pub mod global_windows_payload {
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
        Properties = 0,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Properties => "PROPERTIES",
            }
        }
    }
}
/// A fixed time window represents a consistent duration size, non overlapping
/// time interval in the data stream.
///
/// See <https://beam.apache.org/documentation/programming-guide/#fixed-time-windows>
/// for additional details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedWindowsPayload {
    /// (Required) Represents the size of the window.
    #[prost(message, optional, tag = "1")]
    pub size: ::core::option::Option<::prost_types::Duration>,
    /// (Required) Represents the timestamp of when the first window begins.
    /// Window N will start at offset + N * size.
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `FixedWindowsPayload`.
pub mod fixed_windows_payload {
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
        Properties = 0,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Properties => "PROPERTIES",
            }
        }
    }
}
/// A sliding time window represents time intervals in the data stream that can
/// overlap. For example, each window might capture 60 seconds worth of data, but
/// a new window starts every 30 seconds. The frequency with which sliding
/// windows begin is called the period. Therefore, our example would have a
/// window size of 60 seconds and a period of 30 seconds.
///
/// Because multiple windows overlap, most elements in a data set will belong to
/// more than one window. This kind of windowing is useful for taking running
/// averages of data; using sliding time windows, you can compute a running
/// average of the past 60 seconds’ worth of data, updated every 30 seconds, in
/// our example.
///
/// See <https://beam.apache.org/documentation/programming-guide/#sliding-time-windows>
/// for additional details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlidingWindowsPayload {
    /// (Required) Represents the size of the window.
    #[prost(message, optional, tag = "1")]
    pub size: ::core::option::Option<::prost_types::Duration>,
    /// (Required) Represents the timestamp of when the first window begins.
    /// Window N will start at offset + N * period.
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<::prost_types::Timestamp>,
    /// (Required) Represents the amount of time between each start of a window.
    #[prost(message, optional, tag = "3")]
    pub period: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `SlidingWindowsPayload`.
pub mod sliding_windows_payload {
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
        Properties = 0,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Properties => "PROPERTIES",
            }
        }
    }
}
/// A session window function defines windows that contain elements that are
/// within a certain gap size of another element. Session windowing applies
/// on a per-key basis and is useful for data that is irregularly distributed
/// with respect to time. For example, a data stream representing user mouse
/// activity may have long periods of idle time interspersed with high
/// concentrations of clicks. If data arrives after the minimum specified gap
/// size duration, this initiates the start of a new window.
///
/// See <https://beam.apache.org/documentation/programming-guide/#session-windows>
/// for additional details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionWindowsPayload {
    /// (Required) Minimum duration of gaps between sessions.
    #[prost(message, optional, tag = "1")]
    pub gap_size: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `SessionWindowsPayload`.
pub mod session_windows_payload {
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
        Properties = 0,
    }
    impl Enum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Enum::Properties => "PROPERTIES",
            }
        }
    }
}
