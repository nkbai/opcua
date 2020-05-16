// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    string::UAString,
    localized_text::LocalizedText,
    node_id::NodeId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StructureField {
    pub name: UAString,
    pub description: LocalizedText,
    pub data_type: NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub max_string_length: u32,
    pub is_optional: bool,
}

impl MessageInfo for StructureField {
    fn object_id(&self) -> ObjectId {
        ObjectId::StructureField_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<StructureField> for StructureField {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.name.byte_len();
        size += self.description.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += byte_len_array(&self.array_dimensions);
        size += self.max_string_length.byte_len();
        size += self.is_optional.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += write_array(stream, &self.array_dimensions)?;
        size += self.max_string_length.encode(stream)?;
        size += self.is_optional.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let name = UAString::decode(stream, decoding_limits)?;
        let description = LocalizedText::decode(stream, decoding_limits)?;
        let data_type = NodeId::decode(stream, decoding_limits)?;
        let value_rank = i32::decode(stream, decoding_limits)?;
        let array_dimensions: Option<Vec<u32>> = read_array(stream, decoding_limits)?;
        let max_string_length = u32::decode(stream, decoding_limits)?;
        let is_optional = bool::decode(stream, decoding_limits)?;
        Ok(StructureField {
            name,
            description,
            data_type,
            value_rank,
            array_dimensions,
            max_string_length,
            is_optional,
        })
    }
}
