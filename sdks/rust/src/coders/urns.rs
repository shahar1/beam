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

use strum::{EnumDiscriminants, EnumIter};

#[derive(Debug, EnumDiscriminants, EnumIter)]
pub(crate) enum PresetCoderUrn {
    Bytes,
    Kv,
    Iterable,
    StrUtf8,
    VarInt,
    Unit,
    GeneralObject,
}

impl PresetCoderUrn {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            // ******* Standard coders *******
            Self::Bytes => BYTES_CODER_URN,
            Self::Kv => KV_CODER_URN,
            Self::Iterable => ITERABLE_CODER_URN,

            // ******* Required coders *******
            Self::StrUtf8 => STR_UTF8_CODER_URN,
            Self::VarInt => VARINT_CODER_URN,

            // ******* Rust coders *******
            Self::Unit => UNIT_CODER_URN,
            Self::GeneralObject => GENERAL_OBJECT_CODER_URN,
        }
    }
}

impl AsRef<str> for PresetCoderUrn {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// ******* Standard coders *******
pub const BYTES_CODER_URN: &str = "beam:coder:bytes:v1";
pub const KV_CODER_URN: &str = "beam:coder:kvcoder:v1";
pub const ITERABLE_CODER_URN: &str = "beam:coder:iterable:v1";

// ******* Required coders *******
pub const STR_UTF8_CODER_URN: &str = "beam:coder:string_utf8:v1";
pub const VARINT_CODER_URN: &str = "beam:coder:varint:v1";

// ******* Rust coders *******
pub const UNIT_CODER_URN: &str = "beam:coder:rustsdk:1.0:unit:v1";
pub const GENERAL_OBJECT_CODER_URN: &str = "beam:coder:rustsdk:1.0:genericobject:v1";
