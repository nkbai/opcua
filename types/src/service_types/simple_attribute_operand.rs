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
    node_id::NodeId,
    qualified_name::QualifiedName,
    string::UAString,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SimpleAttributeOperand {
    pub type_definition_id: NodeId,
    pub browse_path: Option<Vec<QualifiedName>>,
    pub attribute_id: u32,
    pub index_range: UAString,
}

impl BinaryEncoder<SimpleAttributeOperand> for SimpleAttributeOperand {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.type_definition_id.byte_len();
        size += byte_len_array(&self.browse_path);
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.type_definition_id.encode(stream)?;
        size += write_array(stream, &self.browse_path)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let type_definition_id = NodeId::decode(stream, decoding_limits)?;
        let browse_path: Option<Vec<QualifiedName>> = read_array(stream, decoding_limits)?;
        let attribute_id = u32::decode(stream, decoding_limits)?;
        let index_range = UAString::decode(stream, decoding_limits)?;
        Ok(SimpleAttributeOperand {
            type_definition_id,
            browse_path,
            attribute_id,
            index_range,
        })
    }
}
